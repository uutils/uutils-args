mod argument;
mod attributes;
mod flags;
mod help;
mod help_parser;
mod initial;

use argument::{
    free_handling, long_handling, parse_argument, parse_arguments_attr, positional_handling,
    short_handling,
};
use attributes::ValueAttr;
use help::{help_handling, help_string, version_handling};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data::Enum, DeriveInput};

#[proc_macro_derive(Initial, attributes(initial))]
pub fn initial(input: TokenStream) -> TokenStream {
    initial::initial(input)
}

#[proc_macro_derive(Arguments, attributes(flag, option, positional, free, arguments))]
pub fn arguments(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Enum(data) = input.data else {
        panic!("Input should be an enum!");
    };

    let arguments_attr = parse_arguments_attr(&input.attrs);
    let arguments: Vec<_> = data.variants.into_iter().flat_map(parse_argument).collect();

    let exit_code = arguments_attr.exit_code;
    let (short, short_flags) = short_handling(&arguments);
    let long = long_handling(&arguments, &arguments_attr.help_flags);
    // let number_argument = number_handling(&arguments);
    let free = free_handling(&arguments);
    let (positional, missing_argument_checks) = positional_handling(&arguments);
    let help_string = help_string(
        &arguments,
        &arguments_attr.help_flags,
        &arguments_attr.version_flags,
        &arguments_attr.file,
    );
    let help = help_handling(&arguments_attr.help_flags);
    let version = version_handling(&arguments_attr.version_flags);
    let version_string = quote!(format!(
        "{} {}",
        option_env!("CARGO_BIN_NAME").unwrap_or(env!("CARGO_PKG_NAME")),
        env!("CARGO_PKG_VERSION"),
    ));

    // This is a bit of a hack to support `echo` and should probably not be
    // used in general.
    let next_arg = if arguments_attr.parse_echo_style {
        quote!(if let Some(val) = uutils_args::__echo_style_positional(parser, &[#(#short_flags),*]) {
            Some(lexopt::Arg::Value(val))
        } else {
            parser.next()?
        })
    } else {
        quote!(parser.next()?)
    };

    let expanded = quote!(
        impl #impl_generics Arguments for #name #ty_generics #where_clause {
            const EXIT_CODE: i32 = #exit_code;

            #[allow(unreachable_code)]
            fn next_arg(
                parser: &mut uutils_args::lexopt::Parser, positional_idx: &mut usize
            ) -> Result<Option<uutils_args::Argument<Self>>, uutils_args::Error> {
                use uutils_args::{Value, lexopt, Error, Argument};

                // #number_argment
                #free

                let arg = match { #next_arg } {
                    Some(arg) => arg,
                    None => return Ok(None),
                };

                #help

                #version

                match arg {
                    lexopt::Arg::Short(short) => { #short },
                    lexopt::Arg::Long(long) => { #long },
                    lexopt::Arg::Value(value) => { #positional },
                }
            }

            fn check_missing(positional_idx: usize) -> Result<(), uutils_args::Error> {
                #missing_argument_checks
            }

            fn help(bin_name: &str) -> ::std::io::Result<()> {
                #help_string
            }

            fn version() -> String {
                #version_string
            }
        }
    );

    TokenStream::from(expanded)
}

#[proc_macro_derive(Value, attributes(value))]
pub fn value(input: TokenStream) -> TokenStream {
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
            if !attr.path().is_ident("value") {
                continue;
            }

            let ValueAttr { keys, value } = ValueAttr::parse(&attr).unwrap();

            let keys = if keys.is_empty() {
                vec![variant_name.to_lowercase()]
            } else {
                keys
            };

            options.push(quote!(&[#(#keys),*]));

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
        impl #impl_generics Value for #name #ty_generics #where_clause {
            fn from_value(value: &::std::ffi::OsStr) -> ::uutils_args::ValueResult<Self> {
                let value = String::from_value(value)?;
                let options: &[&[&str]] = &[#(#options),*];
                let mut candidates: Vec<&str> = Vec::new();
                let mut exact_match: Option<&str> = None;

                'outer: for &opt in options {
                    'inner: for &o in opt {
                        if value == o {
                            exact_match = Some(o);
                            break 'outer;
                        } else if o.starts_with(&value) {
                            candidates.push(o);
                            break 'inner;
                        }
                    }
                }

                let opt = match (exact_match, &candidates[..]) {
                    (Some(opt), _) => opt,
                    (None, [opt]) => opt,
                    (None, []) => return Err("Invalid value".into()),
                    (None, opts) => return Err(uutils_args::ValueError::AmbiguousValue {
                        value,
                        candidates: candidates.iter().map(|s| s.to_string()).collect(),
                    }.into())
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
