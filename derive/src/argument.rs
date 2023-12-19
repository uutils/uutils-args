// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Fields, FieldsUnnamed, Ident, Meta, Variant};

use crate::{
    attributes::{ArgAttr, ArgumentsAttr},
    flags::{Flags, Value},
};

pub struct Argument {
    pub ident: Ident,
    pub field: Option<syn::Type>,
    pub name: String,
    pub arg_type: ArgType,
    pub help: String,
}

pub enum ArgType {
    Option {
        flags: Flags,
        hidden: bool,
        takes_value: bool,
        default: TokenStream,
    },
    Free {
        filters: Vec<syn::Ident>,
    },
}

pub fn parse_arguments_attr(attrs: &[Attribute]) -> ArgumentsAttr {
    for attr in attrs {
        if attr.path().is_ident("arguments") {
            return ArgumentsAttr::parse(attr).unwrap();
        }
    }
    ArgumentsAttr::default()
}

pub fn parse_argument(v: Variant) -> Vec<Argument> {
    let ident = v.ident;
    let name = ident.to_string();
    let attributes = get_arg_attributes(&v.attrs).unwrap();

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
                    let default_expr = match opt.value {
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
                ArgAttr::Free(free) => ArgType::Free {
                    filters: free.filters,
                },
            };
            Argument {
                ident: ident.clone(),
                field: field.clone(),
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
        if attr.path().is_ident("doc") {
            let value = match &attr.meta {
                Meta::NameValue(name_value) => &name_value.value,
                _ => panic!("doc attribute must be a name and a value"),
            };
            let lit = match value {
                syn::Expr::Lit(expr_lit) => &expr_lit.lit,
                _ => panic!("argument to doc attribute must be a string literal"),
            };
            let litstr = match lit {
                syn::Lit::Str(litstr) => litstr,
                _ => panic!("argument to doc attribute must be a string literal"),
            };
            help.push(litstr.value().trim().to_string());
        }
    }
    help.join("\n")
}

fn get_arg_attributes(attrs: &[Attribute]) -> syn::Result<Vec<ArgAttr>> {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("arg"))
        .map(ArgAttr::parse)
        .collect()
}

pub fn short_handling(args: &[Argument]) -> (TokenStream, Vec<char>) {
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
            ArgType::Free { .. } => continue,
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
                _ => return Err(::uutils_args::ErrorKind::UnexpectedOption(short.to_string(), Vec::new())),
            }
        )))
    );
    (token_stream, short_flags)
}

pub fn long_handling(args: &[Argument], help_flags: &Flags) -> TokenStream {
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
            ArgType::Free { .. } => continue,
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
        return quote!(
            return Err(::uutils_args::ErrorKind::UnexpectedOption(
                long.to_string(),
                Vec::new()
            ))
        );
    }

    // TODO: Add version check
    let help_check = if !help_flags.long.is_empty() {
        let long_help_flags = help_flags.long.iter().map(|f| &f.flag);
        quote!(if let #(#long_help_flags)|* = long {
            return Ok(Some(::uutils_args::Argument::Help));
        })
    } else {
        quote!()
    };

    let num_opts = options.len();

    quote!(
        let long_options: [&str; #num_opts] = [#(#options),*];
        let long = ::uutils_args::internal::infer_long_option(long, &long_options)?;

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

pub fn free_handling(args: &[Argument]) -> TokenStream {
    let mut if_expressions = Vec::new();

    // Free arguments
    for arg @ Argument { arg_type, .. } in args {
        let filters = match arg_type {
            ArgType::Free { filters } => filters,
            ArgType::Option { .. } => continue,
        };

        for filter in filters {
            let ident = &arg.ident;

            if_expressions.push(quote!(
                if let Some(inner) = #filter(arg) {
                    let value = ::uutils_args::internal::parse_value_for_option("", ::std::ffi::OsStr::new(inner))?;
                    let _ = raw.next();
                    return Ok(Some(Argument::Custom(Self::#ident(value))));
                }
            ));
        }
    }

    // dd-style arguments
    let mut dd_branches = Vec::new();
    let mut dd_args = Vec::new();
    for arg @ Argument { arg_type, .. } in args {
        let flags = match arg_type {
            ArgType::Option { flags, .. } => flags,
            ArgType::Free { .. } => continue,
        };

        for (prefix, _) in &flags.dd_style {
            let ident = &arg.ident;

            dd_args.push(prefix);
            dd_branches.push(quote!(
                if prefix == #prefix {
                    let value = ::uutils_args::internal::parse_value_for_option("", ::std::ffi::OsStr::new(value))?;
                    let _ = raw.next();
                    return Ok(Some(Argument::Custom(Self::#ident(value))));
                }
            ));
        }
    }

    if !dd_branches.is_empty() {
        if_expressions.push(quote!(
            if let Some((prefix, value)) = arg.split_once('=') {
                #(#dd_branches)*

                return Err(::uutils_args::ErrorKind::UnexpectedOption(
                    prefix.to_string(),
                    ::uutils_args::internal::filter_suggestions(prefix, &[#(#dd_args),*], "")
                ));
            }
        ));
    }

    quote!(
        if let Some(mut raw) = parser.try_raw_args() {
            if let Some(arg) = raw.peek().and_then(|s| s.to_str()) {
                #(#if_expressions)*
            }
        }
    )
}

fn no_value_expression(ident: &Ident) -> TokenStream {
    quote!(Self::#ident)
}

fn default_value_expression(ident: &Ident, default_expr: &TokenStream) -> TokenStream {
    quote!(Self::#ident(#default_expr))
}

fn optional_value_expression(ident: &Ident, default_expr: &TokenStream) -> TokenStream {
    quote!(match parser.optional_value() {
        Some(value) => Self::#ident(::uutils_args::internal::parse_value_for_option(&option, &value)?),
        None => Self::#ident(#default_expr),
    })
}

fn required_value_expression(ident: &Ident) -> TokenStream {
    quote!(Self::#ident(::uutils_args::internal::parse_value_for_option(&option, &parser.value()?)?))
}
