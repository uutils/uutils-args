mod action;
mod attributes;

use action::{parse_action_attr, ActionAttr, ActionType};
use attributes::{
    parse_flag_attr, parse_option_attr, parse_value_attr, FlagAttr, OptionAttr, ValueAttr,
};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse,
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

#[proc_macro_derive(Options, attributes(arg_type, map, set, set_true, set_false, collect))]
pub fn options(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let arg_type = input
        .attrs
        .iter()
        .find(|a| a.path.is_ident("arg_type"))
        .expect("An Options struct must have a `arg_type` attribute")
        .parse_args_with(syn::Ident::parse)
        .expect("The `arg_type` attribute must contain a valid identifier.");
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Struct(data) = input.data else {
        panic!("Input should be a struct!");
    };

    let Fields::Named(fields) = data.fields else {
        panic!("Fields must be named");
    };

    // The key of this map is a literal pattern and the value
    // is whatever code needs to be run when that pattern is encountered.
    let mut stmts = Vec::new();

    for field in fields.named {
        let field_ident = field.ident.as_ref().unwrap();
        for attr in field.attrs {
            let Some(ActionAttr { action_type, collect }) = parse_action_attr(attr) else { continue; };

            let mut match_arms = vec![];
            match action_type {
                ActionType::Map(arms) => {
                    for arm in arms {
                        let pat = arm.pat;
                        let expr = arm.body;
                        match_arms.push((quote!(#pat), quote!(#expr.clone())));
                    }
                }

                ActionType::Set(pats) => {
                    let pats: Vec<_> = pats.iter().map(|p| quote!(#p(x))).collect();
                    let pats = quote!(#(#pats)|*);
                    match_arms.push((pats, quote!(x.clone())))
                }

                ActionType::SetTrue(pats) => {
                    let pats = quote!(#(#pats)|*);
                    match_arms.push((pats, quote!(true)))
                }

                ActionType::SetFalse(pats) => {
                    let pats = quote!(#(#pats)|*);
                    match_arms.push((pats, quote!(false)))
                }
            };

            for (pat, expr) in match_arms {
                stmts.push(if collect {
                    quote!(
                        if let #pat = &arg {
                            self.#field_ident.push(#expr);
                        }
                    )
                } else {
                    quote!(
                        if let #pat = &arg {
                            self.#field_ident = #expr;
                        }
                    )
                })
            }
        }
    }

    let expanded = quote!(
        impl #impl_generics Options for #name #ty_generics #where_clause {
            fn apply_args<I>(&mut self, args: I) -> Result<(), uutils_args::Error>
            where
                I: IntoIterator + 'static,
                I::Item: Into<std::ffi::OsString>,
            {
                use uutils_args::lexopt;
                use uutils_args::FromValue;
                let mut iter = #arg_type::parse(args);
                while let Some(arg) = iter.next_arg()? {
                    #(#stmts)*
                }
                Ok(())
            }
        }
    );

    TokenStream::from(expanded)
}

#[proc_macro_derive(Arguments, attributes(flag, option))]
pub fn arguments(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Enum(data) = input.data else {
        panic!("Input should be an enum!");
    };

    let mut match_arms = Vec::new();

    for variant in data.variants {
        let variant_ident = variant.ident;
        let variant_name = variant_ident.to_string();
        let mut short_flags = Vec::new();
        let mut long_flags = Vec::new();
        let parse_expr = match variant.fields {
            Fields::Unit => {
                for attr in variant.attrs {
                    let Some(attr) = parse_attr(attr) else { continue; };
                    let DeriveAttribute::Flag(f) = attr else {
                        panic!("unsupported attribute");
                    };
                    let (mut shorts, mut longs) = flag_names(f.flags, &variant_name);
                    short_flags.append(&mut shorts);
                    long_flags.append(&mut longs);
                }
                quote!(Self::#variant_ident)
            }
            Fields::Unnamed(f) => {
                let v: Vec<_> = f.unnamed.iter().collect();
                assert_eq!(v.len(), 1, "Options can have only one field");
                for attr in variant.attrs {
                    let Some(attr) = parse_attr(attr) else { continue; };
                    let DeriveAttribute::Option(f) = attr else {
                        panic!("unsupported attribute");
                    };
                    let (mut shorts, mut longs) = flag_names(f.flags, &variant_name);
                    short_flags.append(&mut shorts);
                    long_flags.append(&mut longs);
                }
                quote!(Self::#variant_ident (FromValue::from_value(parser.value()?)?))
            }
            _ => panic!("unimplemented"),
        };

        let short_pattern = if short_flags.is_empty() {
            None
        } else {
            Some(quote!(uutils_args::lexopt::Arg::Short(#(#short_flags)|*)))
        };

        let long_pattern = if long_flags.is_empty() {
            None
        } else {
            Some(quote!(uutils_args::lexopt::Arg::Long(#(#long_flags)|*)))
        };

        let pattern = match (short_pattern, long_pattern) {
            // No flags given, so we just ignore this variant,
            // could be user error though, so we might want to
            // panic
            (None, None) => continue,
            (Some(s), None) => s,
            (None, Some(l)) => l,
            (Some(s), Some(l)) => quote!(#s | #l),
        };

        match_arms.push(quote!(#pattern => { #parse_expr }))
    }

    let expanded = quote!(
        impl #impl_generics Arguments for #name #ty_generics #where_clause {
            fn next_arg(parser: &mut uutils_args::lexopt::Parser) -> Result<Option<Self>, uutils_args::Error> {
                use uutils_args::FromValue;
                let Some(arg) = parser.next()? else { return Ok(None); };
                Ok(Some(match arg {
                    #(#match_arms)*
                    _ => return Err(arg.unexpected().into())
                }))
            }
        }
    );

    TokenStream::from(expanded)
}

#[proc_macro_derive(FromValue, attributes(value))]
pub fn from_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Enum(data) = input.data else {
        panic!("Input should be an enum!");
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
                use uutils_args::FromValue;
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

fn flag_names(flags: Vec<Arg>, field_name: &str) -> (Vec<char>, Vec<String>) {
    let field_name = field_name.to_lowercase();
    if flags.is_empty() {
        let first_char = field_name.chars().next().unwrap();
        if field_name.len() > 1 {
            (vec![first_char], vec![field_name.to_string()])
        } else {
            (vec![first_char], vec![])
        }
    } else {
        let mut shorts = Vec::new();
        let mut longs = Vec::new();
        for flag in flags {
            match flag {
                Arg::Short(x) => shorts.push(x),
                Arg::Long(x) => longs.push(x),
            };
        }
        (shorts, longs)
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
