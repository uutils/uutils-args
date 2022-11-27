use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute,
    Data::Struct,
    DeriveInput, Fields, LitStr, Token,
};

#[derive(Eq, Hash, PartialEq, Debug)]
enum Arg {
    Short(char),
    Long(String),
}

enum OptionsAttribute {
    Flag(FlagAttribute),
}

struct FlagAttribute {
    flags: Vec<Arg>,
}

enum FlagArg {
    Short(char),
    Long(String),
}

// FIXME: Think of a better name
#[proc_macro_derive(Options, attributes(flag))]
pub fn options(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Struct(data) = input.data else {
        panic!("Input should be a struct!");
    };

    let Fields::Named(fields) = data.fields else {
        panic!("Fields must be named");
    };

    // The key of this map is a literal pattern and the value
    // is whatever code needs to be run when that pattern is encountered.
    let mut map: HashMap<Arg, Vec<TokenStream2>> = HashMap::new();

    for field in fields.named {
        let field_ident = field.ident.as_ref().expect("Each field must be named.");
        let field_name = field_ident.to_string();
        let field_char = field_name.chars().next().unwrap();
        for attr in field.attrs {
            let Some(attr) = parse_attr(attr) else { continue; };
            match attr {
                OptionsAttribute::Flag(f) => {
                    let flags = if f.flags.is_empty() {
                        if field_name.len() > 1 {
                            vec![Arg::Short(field_char), Arg::Long(field_name.clone())]
                        } else {
                            vec![Arg::Short(field_char)]
                        }
                    } else {
                        f.flags
                    };
                    for flag in flags {
                        map.entry(flag)
                            .or_default()
                            .push(quote!(self.#field_ident = true;));
                    }
                }
            }
        }
    }

    let mut match_arms = vec![];
    for (pattern, arms) in map {
        match pattern {
            Arg::Short(char) => match_arms.push(quote!(lexopt::Arg::Short(#char) => {#(#arms)*})),
            Arg::Long(string) => match_arms.push(quote!(lexopt::Arg::Long(#string) => {#(#arms)*})),
        }
    }

    let expanded = quote!(
        impl #impl_generics Options for #name #ty_generics #where_clause {
            fn apply_args<I>(&mut self, args: I) -> Result<(), lexopt::Error>
            where
                I: IntoIterator + 'static,
                I::Item: Into<std::ffi::OsString>,
            {
                use uutils_args::lexopt;
                let mut parser = lexopt::Parser::from_args(args);
                while let Some(arg) = parser.next()? {
                    match arg {
                        #(#match_arms)*,
                        _ => { return Err(arg.unexpected());}
                    }
                }
                Ok(())
            }
        }
    );

    TokenStream::from(expanded)
}

fn parse_attr(attr: Attribute) -> Option<OptionsAttribute> {
    if attr.path.is_ident("flag") {
        return Some(OptionsAttribute::Flag(parse_flag_attr(attr)));
    }
    None
}

fn parse_flag_attr(attr: Attribute) -> FlagAttribute {
    let mut flag_attr = FlagAttribute { flags: vec![] };
    let Ok(parsed_args) = attr
        .parse_args_with(Punctuated::<FlagArg, Token![,]>::parse_terminated)
    else {
        return flag_attr;
    };
    for arg in parsed_args {
        match arg {
            FlagArg::Long(s) => flag_attr.flags.push(Arg::Long(s)),
            FlagArg::Short(c) => flag_attr.flags.push(Arg::Short(c)),
        };
    }
    flag_attr
}

impl Parse for FlagArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            let str = input.parse::<LitStr>().unwrap().value();
            if let Some(s) = str.strip_prefix("--") {
                return Ok(FlagArg::Long(s.to_owned()));
            } else if let Some(s) = str.strip_prefix('-') {
                assert_eq!(
                    s.len(),
                    1,
                    "Exactly one character must follow '-' in a flag attribute"
                );
                return Ok(FlagArg::Short(s.chars().next().unwrap()));
            }
            panic!("Arguments to flag must start with \"-\" or \"--\"");
        }
        panic!("Arguments to flag attribute must be string literals");
    }
}
