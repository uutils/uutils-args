mod action;
mod argument;
mod attributes;
mod field;
mod flags;
mod help;
mod markdown;

use argument::{long_handling, parse_argument, positional_handling, short_handling};
use attributes::ValueAttr;
use field::{parse_field, FieldData};
use help::{help_handling, help_string, parse_help_attr, parse_version_attr, version_handling};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse,
    parse_macro_input,
    Data::{Enum, Struct},
    DeriveInput, Fields,
};

#[proc_macro_derive(Options, attributes(arg_type, map, set, field, collect))]
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
    let mut defaults = Vec::new();
    for field in fields.named {
        let FieldData {
            ident,
            default_value,
            match_stmt,
        } = parse_field(&field);

        defaults.push(quote!(#ident: #default_value));
        stmts.push(match_stmt);
    }

    let expanded = quote!(
        impl #impl_generics Options for #name #ty_generics #where_clause {
            fn initial() -> Result<Self, uutils_args::Error> {
                Ok(Self {
                    #(#defaults),*
                })
            }

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
                            print!("{}", iter.help());
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

    let mut options = Vec::new();

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

            options.extend_from_slice(&keys);

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

    let num_opts = options.len();
    let expanded = quote!(
        impl #impl_generics FromValue for #name #ty_generics #where_clause {
            fn from_value(option: &str, value: std::ffi::OsString) -> Result<Self, uutils_args::Error> {
                let value = String::from_value(option, value)?;
                let options: [&str; #num_opts] = [#(#options),*];
                let mut candidates = Vec::new();
                let mut exact_match = None;

                for opt in options {
                    if opt == value {
                        exact_match = Some(opt);
                        break;
                    } else if opt.starts_with(&value) {
                        candidates.push(opt);
                    }
                }

                let opt = match (exact_match, &candidates[..]) {
                    (Some(opt), _) => opt,
                    (None, [opt]) => opt,
                    (None, []) => return Err(uutils_args::Error::ParsingFailed {
                        option: option.to_string(),
                        value,
                        error: "Invalid value".into(),
                    }),
                    (None, opts) => return Err(uutils_args::Error::AmbiguousValue {
                        option: option.to_string(),
                        value,
                        candidates: candidates.iter().map(|s| s.to_string()).collect(),
                    })
                };

                Ok(match opt {
                    #(#match_arms),*,
                    _ => unreachable!("Should be caught by (None, []) case above.")
                })
            }
        }
    );

    TokenStream::from(expanded)
}
