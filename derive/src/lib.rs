mod attributes;
use attributes::{
    parse_flag_attr, parse_option_attr, parse_value_attr, FlagAttr, OptionAttr, ValueAttr,
};

use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, Attribute,
    Data::{Enum, Struct},
    DeriveInput, Expr, Fields,
};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
enum Arg {
    Short(char),
    Long(String),
}

enum DeriveAttribute {
    Flag(FlagAttr),
    Option(OptionAttr),
}

// FIXME: Think of a better name
#[proc_macro_derive(Options, attributes(flag, option))]
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
        for attr in field.attrs {
            let Some(attr) = parse_attr(attr) else { continue; };
            match attr {
                DeriveAttribute::Flag(f) => {
                    let stmt = match f.value {
                        Some(e) => quote!(self.#field_ident = #e;),
                        None => quote!(self.#field_ident = true;),
                    };

                    let flags = flag_names(f.flags, &field_name);
                    for flag in flags {
                        map.entry(flag).or_default().push(stmt.clone());
                    }
                }
                DeriveAttribute::Option(o) => {
                    let stmt = match o.parser {
                        Some(e) => quote!(self.#field_ident = #e(parser.value()?)?;),
                        None => {
                            quote!(self.#field_ident = FromValue::from_value(parser.value()?)?;)
                        }
                    };

                    let flags = flag_names(o.flags, &field_name);
                    for flag in flags {
                        map.entry(flag).or_default().push(stmt.clone());
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
                use uutils_args::FromValue;
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

#[proc_macro_derive(FromValue, attributes(value))]
pub fn from_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Enum(data) = input.data else {
        panic!("Input should be a struct!");
    };

    let mut match_arms = vec![];
    for variant in data.variants {
        let variant_name = variant.ident.to_string();
        let attrs = variant.attrs.clone();
        for attr in attrs {
            if !attr.path.is_ident("value") {
                continue;
            }

            let ValueAttr { keys, value } = parse_value_attr(attr);

            let keys = if keys.is_empty() {
                vec![variant_name.to_lowercase()]
            } else {
                keys
            };

            let stmt = if let Some(v) = value {
                quote!(#(| #keys)* => #v)
            } else {
                let mut v = variant.clone();
                v.attrs = vec![];
                quote!(#(| #keys)* => Self::#v)
            };
            match_arms.push(stmt);
        }
    }

    let expanded = quote!(
        impl #impl_generics FromValue for #name #ty_generics #where_clause {
            fn from_value(value: std::ffi::OsString) -> Result<Self, lexopt::Error> {
                let value = value.into_string()?;
                Ok(match value.as_str() {
                    #(#match_arms),*,
                    _ => {
                        return Err(lexopt::Error::ParsingFailed {
                            value,
                            error: "Invalid value".into(),
                        });
                    }
                })
            }
        }
    );

    TokenStream::from(expanded)
}

fn flag_names(flags: Vec<Arg>, field_name: &str) -> Vec<Arg> {
    if flags.is_empty() {
        let first_char = field_name.chars().next().unwrap();
        if field_name.len() > 1 {
            vec![Arg::Short(first_char), Arg::Long(field_name.to_string())]
        } else {
            vec![Arg::Short(first_char)]
        }
    } else {
        flags
    }
}

fn parse_attr(attr: Attribute) -> Option<DeriveAttribute> {
    if attr.path.is_ident("flag") {
        Some(DeriveAttribute::Flag(parse_flag_attr(attr)))
    } else if attr.path.is_ident("option") {
        Some(DeriveAttribute::Option(parse_option_attr(attr)))
    } else {
        None
    }
}
