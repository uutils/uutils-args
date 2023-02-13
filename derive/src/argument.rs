use std::ops::RangeInclusive;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Fields, FieldsUnnamed, Ident, Lit, Meta, Variant};

use crate::{
    attributes::{parse_argument_attribute, ArgAttr, ArgumentsAttr},
    flags::{Flag, Flags, Value},
};

pub(crate) struct Argument {
    pub(crate) ident: Ident,
    pub(crate) name: String,
    pub(crate) arg_type: ArgType,
    pub(crate) help: String,
}

pub(crate) enum ArgType {
    Option {
        flags: Flags,
        hidden: bool,
        takes_value: bool,
        default: TokenStream,
    },
    Positional {
        num_args: RangeInclusive<usize>,
        last: bool,
    },
}

pub(crate) fn parse_arguments_attr(attrs: &[Attribute]) -> ArgumentsAttr {
    for attr in attrs {
        if attr.path.is_ident("arguments") {
            return ArgumentsAttr::parse(attr);
        }
    }
    ArgumentsAttr::default()
}

pub(crate) fn parse_argument(v: Variant) -> Vec<Argument> {
    let ident = v.ident;
    let name = ident.to_string();
    let attributes = get_arg_attributes(&v.attrs);

    // Return early because we don't need to check the fields if it's not used.
    if attributes.is_empty() {
        return Vec::new();
    }

    let help = collect_help(&v.attrs);

    let field = match v.fields {
        Fields::Unit => None,
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let v: Vec<_> = unnamed.iter().collect();
            assert!(
                v.len() == 1,
                "Variants in an Arguments enum can have at most 1 field."
            );
            Some(v[0].ty.clone())
        }
        Fields::Named(_) => {
            panic!("Named fields are not supported in Arguments");
        }
    };

    attributes
        .into_iter()
        .map(|attribute| {
            // We might override the help with the help given in the attribute
            let mut arg_help = help.clone();
            let arg_type = match attribute {
                ArgAttr::Option(opt) => {
                    let default_expr = match opt.default {
                        Some(expr) => quote!(#expr),
                        None => quote!(Default::default()),
                    };
                    if let Some(help) = opt.help {
                        arg_help = help;
                    }
                    ArgType::Option {
                        flags: opt.flags,
                        takes_value: field.is_some(),
                        default: default_expr,
                        hidden: opt.hidden,
                    }
                }
                ArgAttr::Positional(pos) => {
                    assert!(field.is_some(), "Positional arguments must have a field");
                    ArgType::Positional {
                        num_args: pos.num_args,
                        last: pos.last,
                    }
                }
            };
            Argument {
                ident: ident.clone(),
                name: name.clone(),
                arg_type,
                help: arg_help,
            }
        })
        .collect()
}

fn collect_help(attrs: &[Attribute]) -> String {
    let mut help = Vec::new();
    for attr in attrs {
        let Ok(meta) = attr.parse_meta() else { continue; };
        let Meta::NameValue(name_value) = meta else { continue; };
        if !name_value.path.is_ident("doc") {
            continue;
        }
        let Lit::Str(litstr) = name_value.lit else { continue; };
        help.push(litstr.value().trim().to_string())
    }
    help.join("\n")
}

fn get_arg_attributes(attrs: &[Attribute]) -> Vec<ArgAttr> {
    attrs
        .iter()
        .filter(|a| a.path.is_ident("option") || a.path.is_ident("positional"))
        .map(parse_argument_attribute)
        .collect()
}

pub(crate) fn short_handling(args: &[Argument]) -> (TokenStream, Vec<char>) {
    let mut match_arms = Vec::new();
    let mut short_flags = Vec::new();

    for arg in args {
        let (flags, takes_value, default) = match arg.arg_type {
            ArgType::Option {
                ref flags,
                takes_value,
                ref default,
                hidden: _,
            } => (flags, takes_value, default),
            ArgType::Positional { .. } => continue,
        };

        if flags.short.is_empty() {
            continue;
        }

        for flag in &flags.short {
            let pat = flag.flag;
            let expr = match (&flag.value, takes_value) {
                (Value::No, false) => no_value_expression(&arg.ident),
                (_, false) => {
                    panic!("Option cannot take a value if the variant doesn't have a field")
                }
                (Value::No, true) => default_value_expression(&arg.ident, default),
                (Value::Optional(_), true) => optional_value_expression(&arg.ident, default),
                (Value::Required(_), true) => required_value_expression(&arg.ident),
            };
            match_arms.push(quote!(#pat => { #expr }));
            short_flags.push(pat);
        }
    }

    let token_stream = quote!(
        let option = format!("-{}", short);
        Ok(Some(Argument::Custom(
            match short {
                #(#match_arms)*
                _ => return Err(Error::UnexpectedOption(short.to_string())),
            }
        )))
    );
    (token_stream, short_flags)
}

pub(crate) fn long_handling(args: &[Argument], help_flags: &Flags) -> TokenStream {
    let mut match_arms = Vec::new();
    let mut options = Vec::new();

    options.extend(help_flags.long.iter().map(|f| f.flag.clone()));

    for arg in args {
        let (flags, takes_value, default) = match &arg.arg_type {
            ArgType::Option {
                flags,
                takes_value,
                ref default,
                hidden: _,
            } => (flags, takes_value, default),
            ArgType::Positional { .. } => continue,
        };

        if flags.long.is_empty() {
            continue;
        }

        for flag in &flags.long {
            let pat = &flag.flag;
            let expr = match (&flag.value, takes_value) {
                (Value::No, false) => no_value_expression(&arg.ident),
                (_, false) => {
                    panic!("Option cannot take a value if the variant doesn't have a field")
                }
                (Value::No, true) => default_value_expression(&arg.ident, default),
                (Value::Optional(_), true) => optional_value_expression(&arg.ident, default),
                (Value::Required(_), true) => required_value_expression(&arg.ident),
            };
            match_arms.push(quote!(#pat => { #expr }));
            options.push(flag.flag.clone());
        }
    }

    if options.is_empty() {
        return quote!(return Err(Error::UnexpectedOption(long.to_string())));
    }

    // TODO: Add version check
    let help_check = if !help_flags.long.is_empty() {
        let long_help_flags = help_flags.long.iter().map(|f| &f.flag);
        quote!(if let #(#long_help_flags)|* = long {
            return Ok(Some(Argument::Help));
        })
    } else {
        quote!()
    };

    let num_opts = options.len();

    quote!(
        let long_options: [&str; #num_opts] = [#(#options),*];
        let mut candidates = Vec::new();
        let mut exact_match = None;
        for opt in long_options {
            if opt == long {
                exact_match = Some(opt);
                break;
            } else if opt.starts_with(long) {
                candidates.push(opt);
            }
        }

        let long = match (exact_match, &candidates[..]) {
            (Some(opt), _) => opt,
            (None, [opt]) => opt,
            (None, []) => return Err(Error::UnexpectedOption(long.to_string())),
            (None, opts) => return Err(Error::AmbiguousOption {
                option: long.to_string(),
                candidates: candidates.iter().map(|s| s.to_string()).collect(),
            })
        };

        #help_check

        let option = format!("--{}", long);
        Ok(Some(Argument::Custom(
            match long {
                #(#match_arms)*
                _ => unreachable!("Should be caught by (None, []) case above.")
            }
        )))
    )
}

pub(crate) fn number_handling(args: &[Argument]) -> TokenStream {
    let mut number_args = Vec::new();

    for arg in args {
        let flags = match &arg.arg_type {
            ArgType::Option { flags, .. } => flags,
            ArgType::Positional { .. } => continue,
        };

        let ident = &arg.ident;

        for Flag { flag: prefix, .. } in &flags.number_prefix {
            number_args.push(quote!(
                if let Some(v) = ::uutils_args::parse_prefix(parser, #prefix) {
                    return Ok(Some(::uutils_args::Argument::Custom(Self::#ident(v))));
                }
            ));
        }
    }

    quote!(#(#number_args)*)
}

pub(crate) fn positional_handling(args: &[Argument]) -> (TokenStream, TokenStream) {
    let mut match_arms = Vec::new();
    // The largest index of the previous argument, so the the argument after this should
    // belong to the next argument.
    let mut last_index = 0;

    // The minimum number of arguments needed to not return a missing argument error.
    let mut minimum_needed = 0;
    let mut missing_argument_checks = vec![];

    for arg @ Argument { name, arg_type, .. } in args {
        let (num_args, last) = match arg_type {
            ArgType::Positional { num_args, last } => (num_args, last),
            ArgType::Option { .. } => continue,
        };

        if *num_args.start() > 0 {
            minimum_needed = last_index + num_args.start();
            missing_argument_checks.push(quote!(if positional_idx < #minimum_needed {
                missing.push(#name);
            }));
        }

        last_index += num_args.end();

        let expr = if *last {
            last_positional_expression(&arg.ident)
        } else {
            positional_expression(&arg.ident)
        };
        match_arms.push(quote!(0..=#last_index => { #expr }));
    }

    let value_handling = quote!(
        *positional_idx += 1;
        Ok(Some(Argument::Custom(
            match positional_idx {
                #(#match_arms)*
                _ => return Err(lexopt::Arg::Value(value).unexpected().into()),
            }
        )))
    );

    let missing_argument_checks = quote!(
        // We have the minimum number of required arguments overall.
        // So we don't need to check the others.
        if positional_idx >= #minimum_needed {
            return Ok(());
        }

        let mut missing: Vec<&str> = vec![];
        #(#missing_argument_checks)*
        if !missing.is_empty() {
            Err(uutils_args::Error::MissingPositionalArguments(
                missing.iter().map(ToString::to_string).collect::<Vec<String>>()
            ))
        } else {
            Ok(())
        }
    );

    (value_handling, missing_argument_checks)
}

fn no_value_expression(ident: &Ident) -> TokenStream {
    quote!(Self::#ident)
}

fn default_value_expression(ident: &Ident, default_expr: &TokenStream) -> TokenStream {
    quote!(Self::#ident(#default_expr))
}

fn optional_value_expression(ident: &Ident, default_expr: &TokenStream) -> TokenStream {
    quote!(match parser.optional_value() {
        Some(value) => Self::#ident(::uutils_args::parse_value_for_option(&option, &value)?),
        None => Self::#ident(#default_expr),
    })
}

fn required_value_expression(ident: &Ident) -> TokenStream {
    quote!(Self::#ident(::uutils_args::parse_value_for_option(&option, &parser.value()?)?))
}

fn positional_expression(ident: &Ident) -> TokenStream {
    // TODO: Add option name in this from_value call
    quote!(
        Self::#ident(::uutils_args::parse_value_for_option("", &value)?)
    )
}

fn last_positional_expression(ident: &Ident) -> TokenStream {
    // TODO: Add option name in this from_value call
    quote!({
        let raw_args = parser.raw_args()?;
        let collection = std::iter::once(value)
            .chain(raw_args)
            .map(|v| ::uutils_args::parse_value_for_option("", &v))
            .collect::<Result<_,_>>()?;
        Self::#ident(collection)
    })
}
