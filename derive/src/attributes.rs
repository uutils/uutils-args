// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::ops::RangeInclusive;

use syn::{
    meta::ParseNestedMeta, parse::ParseStream, Attribute, Expr, ExprLit, ExprRange, Ident, Lit,
    LitInt, LitStr, RangeLimits, Token,
};

use crate::flags::Flags;

pub(crate) enum ArgAttr {
    Option(OptionAttr),
    Positional(PositionalAttr),
    Free(FreeAttr),
}

pub(crate) fn parse_argument_attribute(attr: &Attribute) -> syn::Result<ArgAttr> {
    if attr.path().is_ident("option") {
        Ok(ArgAttr::Option(OptionAttr::parse(attr)?))
    } else if attr.path().is_ident("positional") {
        Ok(ArgAttr::Positional(PositionalAttr::parse(attr)?))
    } else if attr.path().is_ident("free") {
        Ok(ArgAttr::Free(FreeAttr::parse(attr)?))
    } else {
        panic!("Internal error: invalid argument attribute");
    }
}

pub(crate) struct ArgumentsAttr {
    pub(crate) help_flags: Flags,
    pub(crate) version_flags: Flags,
    pub(crate) file: Option<String>,
    pub(crate) exit_code: i32,
    pub(crate) parse_echo_style: bool,
}

fn get_ident(meta: &ParseNestedMeta) -> syn::Result<String> {
    match meta.path.get_ident() {
        Some(ident) => Ok(ident.to_string()),
        None => Err(meta.error("expected an identifier")),
    }
}

fn assert_expr_is_array_of_litstr(expr: Expr, flag: &str) -> syn::Result<Vec<String>> {
    let arr = match expr {
        syn::Expr::Array(arr) => arr,
        _ => {
            return Err(syn::Error::new_spanned(
                expr,
                format!("Argument to `{flag}` must be an array"),
            ))
        }
    };

    let mut strings = Vec::new();
    for elem in arr.elems {
        let val = match elem {
            syn::Expr::Lit(syn::ExprLit {
                attrs: _,
                lit: syn::Lit::Str(litstr),
            }) => litstr.value(),
            _ => {
                return Err(syn::Error::new_spanned(
                    elem,
                    format!("Argument to `{flag}` must be an array of string literals"),
                ))
            }
        };
        strings.push(val);
    }
    Ok(strings)
}

fn parse_args(
    attr: &Attribute,
    mut logic: impl FnMut(ParseStream) -> syn::Result<()>,
) -> syn::Result<()> {
    attr.parse_args_with(|s: ParseStream| loop {
        logic(s)?;
        if s.is_empty() {
            return Ok(());
        }
        s.parse::<Token![,]>()?;
        if s.is_empty() {
            return Ok(());
        }
    })
}

impl Default for ArgumentsAttr {
    fn default() -> Self {
        Self {
            help_flags: Flags::new(["--help"]),
            version_flags: Flags::new(["--version"]),
            file: None,
            exit_code: 1,
            parse_echo_style: false,
        }
    }
}

impl ArgumentsAttr {
    pub(crate) fn parse(attr: &Attribute) -> syn::Result<Self> {
        let mut args = ArgumentsAttr::default();

        attr.parse_nested_meta(|meta| {
            let ident = get_ident(&meta)?;
            match ident.as_str() {
                "help_flags" => {
                    let expr: Expr = meta.value()?.parse()?;
                    let strings = assert_expr_is_array_of_litstr(expr, "help_flags")?;
                    args.help_flags = Flags::new(strings);
                }
                "version_flags" => {
                    let expr: Expr = meta.value()?.parse()?;
                    let strings = assert_expr_is_array_of_litstr(expr, "version_flags")?;
                    args.version_flags = Flags::new(strings);
                }
                "file" => {
                    let s = meta.value()?.parse::<LitStr>()?.value();
                    args.file = Some(s);
                }
                "exit_code" => {
                    let c = meta.value()?.parse::<LitInt>()?.base10_parse()?;
                    args.exit_code = c;
                }
                "parse_echo_style" => {
                    args.parse_echo_style = true;
                }
                _ => return Err(meta.error("unrecognized argument for arguments attribute")),
            };
            Ok(())
        })?;

        Ok(args)
    }
}

#[derive(Default)]
pub(crate) struct OptionAttr {
    pub(crate) flags: Flags,
    pub(crate) parser: Option<Expr>,
    pub(crate) default: Option<Expr>,
    pub(crate) hidden: bool,
    pub(crate) help: Option<String>,
}

impl OptionAttr {
    pub(crate) fn parse(attr: &Attribute) -> syn::Result<Self> {
        let mut option_attr = OptionAttr::default();

        parse_args(attr, |s: ParseStream| {
            if let Ok(litstr) = s.parse::<LitStr>() {
                option_attr.flags.add(&litstr.value());
                return Ok(());
            }

            let ident = s.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "parser" => {
                    s.parse::<Token![=]>()?;
                    let p = s.parse::<Expr>()?;
                    option_attr.parser = Some(p);
                }
                "default" => {
                    s.parse::<Token![=]>()?;
                    let d = s.parse::<Expr>()?;
                    option_attr.default = Some(d);
                }
                "hidden" => {
                    option_attr.hidden = true;
                }
                "help" => {
                    s.parse::<Token![=]>()?;
                    let h = s.parse::<LitStr>()?;
                    option_attr.help = Some(h.value());
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ident,
                        "unrecognized argument for option attribute",
                    ))
                }
            }
            Ok(())
        })?;

        Ok(option_attr)
    }
}

#[derive(Default)]
pub(crate) struct FreeAttr {
    pub(crate) filters: Vec<syn::Ident>,
}

impl FreeAttr {
    pub(crate) fn parse(attr: &Attribute) -> syn::Result<Self> {
        let mut free_attr = FreeAttr::default();

        parse_args(attr, |s: ParseStream| {
            let ident = s.parse::<Ident>()?;
            free_attr.filters.push(ident);
            Ok(())
        })?;

        Ok(free_attr)
    }
}

#[derive(Default)]
pub(crate) struct ValueAttr {
    pub(crate) keys: Vec<String>,
    pub(crate) value: Option<Expr>,
}

impl ValueAttr {
    pub(crate) fn parse(attr: &Attribute) -> syn::Result<Self> {
        let mut value_attr = Self::default();

        // value does not need to take arguments, so short circuit if it does not have one
        if let syn::Meta::Path(_) = &attr.meta {
            return Ok(value_attr);
        }

        parse_args(attr, |s: ParseStream| {
            if let Ok(litstr) = s.parse::<LitStr>() {
                value_attr.keys.push(litstr.value());
                return Ok(());
            }

            let ident = s.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "value" => {
                    s.parse::<Token![=]>()?;
                    let p = s.parse::<Expr>()?;
                    value_attr.value = Some(p);
                }
                _ => return Err(s.error("unrecognized keyword in value attribute")),
            }
            Ok(())
        })?;

        Ok(value_attr)
    }
}

pub(crate) struct PositionalAttr {
    pub(crate) num_args: RangeInclusive<usize>,
    pub(crate) last: bool,
}

impl Default for PositionalAttr {
    fn default() -> Self {
        Self {
            num_args: 1..=1,
            last: false,
        }
    }
}

impl PositionalAttr {
    pub(crate) fn parse(attr: &Attribute) -> syn::Result<Self> {
        let mut positional_attr = Self::default();
        parse_args(attr, |s| {
            if (s.peek(LitInt) && s.peek2(Token![..])) || s.peek(Token![..]) {
                let range = s.parse::<ExprRange>()?;
                // We're dealing with a range
                let from = match range.start.as_deref() {
                    Some(Expr::Lit(ExprLit {
                        lit: Lit::Int(i), ..
                    })) => i.base10_parse::<usize>().unwrap(),
                    None => 0,
                    _ => panic!("Range must consist of usize"),
                };

                let inclusive = matches!(range.limits, RangeLimits::Closed(_));
                let to = match range.end.as_deref() {
                    Some(Expr::Lit(ExprLit {
                        lit: Lit::Int(i), ..
                    })) => {
                        let n = i.base10_parse::<usize>().unwrap();
                        if inclusive {
                            Some(n)
                        } else {
                            Some(n - 1)
                        }
                    }
                    None => None,
                    _ => panic!("Range must consist of usize"),
                };

                positional_attr.num_args = match to {
                    Some(to) => from..=to,
                    None => from..=usize::MAX,
                };
                return Ok(());
            }

            if let Ok(int) = s.parse::<LitInt>() {
                let suffix = int.suffix();
                // FIXME: should be a proper error instead of assert!
                assert!(
                    suffix.is_empty() || suffix == "usize",
                    "The position index must be usize"
                );
                let n = int.base10_parse::<usize>().unwrap();
                positional_attr.num_args = n..=n;
                return Ok(());
            }

            let ident = s.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "last" => positional_attr.last = true,
                _ => return Err(s.error("unrecognized keyword in value attribute")),
            }
            Ok(())
        })?;

        Ok(positional_attr)
    }
}
