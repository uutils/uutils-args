use std::ops::RangeInclusive;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Fields, FieldsUnnamed, Lit, Meta, Variant, Ident, punctuated::Punctuated, LitStr, Token};

use crate::{
    attributes::{parse_argument_attribute, ArgAttr},
    Arg,
};

pub(crate) struct Argument {
    ident: Ident,
    name: String,
    arg_type: ArgType,
    help: String,
}

pub(crate) enum TakesValue {
    Yes,
    Optional,
    No,
}

pub(crate) enum ArgType {
    Option {
        short_flags: Vec<char>,
        long_flags: Vec<String>,
        takes_value: TakesValue,
    },
    Positional {
        num_args: RangeInclusive<usize>,
    },
}

pub(crate) fn parse_help_flags(attrs: &[Attribute]) -> (Vec<char>, Vec<String>) {
    for attr in attrs {
        if attr.path.is_ident("help") {
            let mut short = Vec::new();
            let mut long = Vec::new();
            for s in attr.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated).unwrap() {
                let s = s.value().to_string();
                if let Some(s) = s.strip_prefix("--") {
                    long.push(s.to_string());
                } else if let Some(s) = s.strip_prefix("-") {
                    assert_eq!(s.len(), 1);
                    short.push(s.chars().next().unwrap())
                }
                return (short, long);
            }
        }
    }
    return (vec!['h'], vec!["help".into()])        
}

pub(crate) fn parse_argument(v: Variant) -> Option<Argument> {
    let ident = v.ident;
    let name = ident.to_string();
    let attribute = get_arg_attribute(&v.attrs)?;
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

    let arg_type = match attribute {
        ArgAttr::Option(opt) => {
            let (short_flags, long_flags) = flag_names(opt.flags, &name);
            let takes_value = match field {
                None => TakesValue::No,
                Some(x) if type_is_option(&x) => TakesValue::Optional,
                Some(_) => TakesValue::Yes,
            };
            ArgType::Option {
                short_flags,
                long_flags,
                takes_value,
            }
        }
        ArgAttr::Positional(pos) => {
            assert!(field.is_some(), "Positional arguments must have a field");
            ArgType::Positional {
                num_args: pos.num_args,
            }
        }
    };

    Some(Argument {
        ident,
        name,
        arg_type,
        help,
    })
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
    help.join(" ")
}

fn get_arg_attribute(attrs: &[Attribute]) -> Option<ArgAttr> {
    let attrs: Vec<_> = attrs
        .iter()
        .filter(|a| a.path.is_ident("option") || a.path.is_ident("positional"))
        .collect();
    match attrs[..] {
        [] => None,
        [attr] => Some(parse_argument_attribute(attr)),
        _ => panic!("Can only specify one #[option] or #[positional] per argument variant"),
    }
}

fn type_is_option(syn_type: &syn::Type) -> bool {
    if let syn::Type::Path(field_type_path) = syn_type {
        field_type_path
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            == vec!["Option"]
    } else {
        false
    }
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

pub(crate) fn short_handling(args: &[Argument]) -> TokenStream {
    let mut match_arms = Vec::new();

    for arg in args {
        let ArgType::Option { ref short_flags, .. } = arg.arg_type else { 
            continue; 
        };
        
        if short_flags.is_empty() {
            continue;
        }

        let expr = argument_expression(arg);
        match_arms.push(quote!(#(#short_flags)|* => { #expr }))
    }

    quote!(
        match short {
            #(#match_arms)*
            _ => return Err(arg.unexpected().into()),
        }
    )
}

pub(crate) fn long_handling(args: &[Argument]) -> TokenStream {
    let mut match_arms = Vec::new();
    let mut options = Vec::new();

    for arg in args {
        let ArgType::Option { ref long_flags, .. } = arg.arg_type else { 
            continue; 
        };

        if long_flags.is_empty() {
            continue;
        }

        let expr = argument_expression(arg);
        match_arms.push(quote!(#(#long_flags)|* => { #expr }));
        options.extend(long_flags);
    }

    if options.is_empty() {
        return quote!(return Err(arg.unexpected().into()));
    }
    
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

        let opt = match (exact_match, &candidates[..]) {
            (Some(opt), _) => opt,
            (None, [opt]) => opt,
            (None, []) => return Err(arg.unexpected().into()),
            (None, opts) => return Err(Error::AmbiguousOption {
                option: long.to_string(),
                candidates: candidates.iter().map(|s| s.to_string()).collect(),
            })
        };

        match opt {
            #(#match_arms)*
            _ => unreachable!("Should be caught by (None, []) case above.")
        }
    )
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
        let ArgType::Positional{ ref num_args } = arg_type else { 
            continue; 
        };

        if *num_args.start() > 0 {
            minimum_needed = last_index + num_args.start();
            missing_argument_checks.push(quote!(if positional_idx < #minimum_needed {
                missing.push(#name);
            }));
        }

        last_index += num_args.end();
        
        let expr = argument_expression(arg);
        match_arms.push(quote!(0..=#last_index => { #expr }));
    }
    
    let value_handling = quote!(
        *positional_idx += 1;
        match positional_idx {
            #(#match_arms)*
            _ => return Err(lexopt::Arg::Value(value).unexpected().into()),
        }
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

fn argument_expression(arg: &Argument) -> TokenStream {
    let Argument { ident, arg_type, .. } = arg;
    match arg_type {
        ArgType::Option { takes_value, .. } => match takes_value {
            TakesValue::No => quote!(Self::#ident),
            TakesValue::Optional => quote!(
                Self::#ident(match parser.optional_value() {
                    Some(x) => Some(FromValue::from_value(x)?),
                    None => None,
                })
            ),
            TakesValue::Yes => quote!(
                Self::#ident(FromValue::from_value(parser.value()?)?)
            ),
        }
        ArgType::Positional { .. } => quote!(
            Self::#ident(FromValue::from_value(value)?)
        ),
    }
}

pub(crate) fn help_handling(short_help_flags: &[char], long_help_flags: &[String]) -> TokenStream {
    let pat = match (short_help_flags, long_help_flags) {
        ([], []) => return quote!(),
        (short, []) => quote!(lexopt::Arg::Short(#(#short)|*)),
        ([], long) => quote!(lexopt::Arg::Long(#(#long)|*)),
        (short, long) => quote!(lexopt::Arg::Short(#(#short)|*) | lexopt::Arg::Long(#(#long)|*))
    };
    
    quote!(
        if let #pat = arg {
            return Ok(Some(Argument::Help));
        }
    )
}

pub(crate) fn help_string(args: &[Argument], short_help_flags: &[char], long_help_flags: &[String]) -> String {
    let mut options = Vec::new();
    
    let width = 16;
    let indent = 2;

    for Argument { arg_type, help, ..} in args {
        match arg_type {
            ArgType::Option { short_flags, long_flags, ..} => {
                let flags = format_flags(short_flags, long_flags);
                options.push(format_help_line(indent, width, &flags, help));
            }
            ArgType::Positional { .. } => {}
        }
    }
    
    let help_flags = format_flags(short_help_flags, long_help_flags);
    if !help_flags.is_empty() {
        options.push(format_help_line(indent, width, &help_flags, "Display this help message"));       
    }

    format!(
        "Options:\n{}",
        options.join("\n"),
    )
}

fn format_flags(short: &[char], long: &[String]) -> String {
    short.iter().map(|s| format!("-{s}"))
        .chain(
            long.iter().map(|l| format!("--{l}"))
        ).collect::<Vec<_>>()
        .join(", ")
}

fn format_help_line(indent: usize, width: usize, flags: &str, help: &str) -> String {
    let indent = " ".repeat(indent);
    if help == "" {
        format!("{indent}{flags}")
    } else if flags.len() >= width {
        let help_indent = " ".repeat(width);
        format!("{indent}{flags}\n{indent}{help_indent}{help}")
    } else {
        let help_indent = " ".repeat(width-flags.len());
        format!("{indent}{flags}{help_indent}{help}")
    }
}
