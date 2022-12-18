mod action;
mod argument;
mod attributes;
mod flags;
mod help;
mod markdown;

use action::{parse_action_attr, ActionAttr, ActionType};
use argument::{
    long_handling, parse_argument, positional_handling, short_handling, version_handling,
};
use attributes::ValueAttr;
use help::{help_handling, help_string, parse_help_attr, parse_version_attr};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse,
    parse_macro_input,
    Data::{Enum, Struct},
    DeriveInput, Fields,
};

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
        let mut match_arms = vec![];
        for attr in field.attrs {
            let Some(ActionAttr { action_type, collect }) = parse_action_attr(attr) else { continue; };

            let mut patterns_and_expressions = vec![];
            match action_type {
                ActionType::Map(arms) => {
                    for arm in arms {
                        let pat = arm.pat;
                        let expr = arm.body;
                        patterns_and_expressions.push((quote!(#pat), quote!(#expr)));
                    }
                }

                ActionType::Set(pats) => {
                    let pats: Vec<_> = pats.iter().map(|p| quote!(#p(x))).collect();
                    let pats = quote!(#(#pats)|*);
                    patterns_and_expressions.push((pats, quote!(x.clone())))
                }
            };
            for (pat, expr) in patterns_and_expressions {
                match_arms.push(if collect {
                    quote!(
                        #pat => { self.#field_ident.push(#expr) }
                    )
                } else {
                    quote!(
                        #pat => { self.#field_ident = #expr }
                    )
                });
            }
        }

        stmts.push(quote!(match arg.clone() {
            #(#match_arms)*
            _ => {}
        }))
    }

    let expanded = quote!(
        impl #impl_generics Options for #name #ty_generics #where_clause {
            fn apply_args<I>(&mut self, args: I) -> Result<(), uutils_args::Error>
            where
                I: IntoIterator + 'static,
                I::Item: Into<std::ffi::OsString>,
            {
                use uutils_args::{lexopt, FromValue, Argument};
                let mut iter = #arg_type::parse(args);
                while let Some(arg) = iter.next_arg()? {
                    match arg {
                        Argument::Help => {
                            println!("{}", iter.help());
                            std::process::exit(0);
                        },
                        Argument::Version => {
                            println!("{}", iter.version());
                        },
                        Argument::Custom(arg) => {
                            #(#stmts)*
                        }
                    }
                }
                #arg_type::check_missing(iter.positional_idx)?;
                Ok(())
            }
        }
    );

    TokenStream::from(expanded)
}

#[proc_macro_derive(Arguments, attributes(flag, option, positional, help, version))]
pub fn arguments(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Enum(data) = input.data else {
        panic!("Input should be an enum!");
    };

    let help_attr = parse_help_attr(&input.attrs);
    let version_attr = parse_version_attr(&input.attrs);
    let arguments: Vec<_> = data.variants.into_iter().flat_map(parse_argument).collect();

    let short = short_handling(&arguments);
    let long = long_handling(&arguments, &help_attr.flags);
    let (positional, missing_argument_checks) = positional_handling(&arguments);
    let help_string = help_string(&arguments, &help_attr, &version_attr.flags);
    let help = help_handling(&help_attr.flags);
    let version = version_handling(&version_attr.flags);
    let version_string = quote!(format!(
        "{} {}",
        option_env!("CARGO_BIN_NAME").unwrap_or(env!("CARGO_PKG_NAME")),
        env!("CARGO_PKG_VERSION"),
    ));

    let expanded = quote!(
        impl #impl_generics Arguments for #name #ty_generics #where_clause {
            #[allow(unreachable_code)]
            fn next_arg(
                parser: &mut uutils_args::lexopt::Parser, positional_idx: &mut usize
            ) -> Result<Option<uutils_args::Argument<Self>>, uutils_args::Error> {
                use uutils_args::{FromValue, lexopt, Error, Argument};

                let Some(arg) = parser.next()? else { return Ok(None); };

                #help

                #version

                let parsed = match arg {
                    lexopt::Arg::Short(short) => { #short }
                    lexopt::Arg::Long(long) => { #long }
                    lexopt::Arg::Value(value) => { #positional }
                };
                Ok(Some(Argument::Custom(parsed)))
            }

            fn check_missing(positional_idx: usize) -> Result<(), uutils_args::Error> {
                #missing_argument_checks
            }

            fn help(bin_name: &str) -> String {
                #help_string
            }

            fn version() -> String {
                #version_string
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

            let ValueAttr { keys, value } = ValueAttr::parse(&attr);

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
