// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use syn::{
    Attribute, Expr, Ident, LitInt, LitStr, Token, meta::ParseNestedMeta, parse::ParseStream,
};

use crate::flags::Flags;

pub struct ArgumentsAttr {
    pub help_flags: Flags,
    pub version_flags: Flags,
    pub file: Option<String>,
    pub exit_code: i32,
    pub parse_echo_style: bool,
    pub options_first: bool,
}

impl Default for ArgumentsAttr {
    fn default() -> Self {
        Self {
            help_flags: Flags::new(["--help"]),
            version_flags: Flags::new(["--version"]),
            file: None,
            exit_code: 1,
            parse_echo_style: false,
            options_first: false,
        }
    }
}

impl ArgumentsAttr {
    pub fn parse(attr: &Attribute) -> syn::Result<Self> {
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
                "options_first" => {
                    args.options_first = true;
                }
                _ => return Err(meta.error("unrecognized argument for arguments attribute")),
            };
            Ok(())
        })?;

        Ok(args)
    }
}

#[allow(clippy::large_enum_variant)]
pub enum ArgAttr {
    Option(OptionAttr),
    Free(FreeAttr),
}

impl ArgAttr {
    pub fn parse(attr: &Attribute) -> syn::Result<Self> {
        assert!(attr.path().is_ident("arg"));

        attr.parse_args_with(|s: ParseStream| {
            // Based on the first value, we determine the type of argument.
            if let Ok(litstr) = s.parse::<LitStr>() {
                let v = litstr.value();
                if v.starts_with('-') || v.contains('=') {
                    OptionAttr::from_args(v, s).map(Self::Option)
                } else {
                    panic!("Could not determine type of argument");
                }
            } else if let Ok(v) = s.parse::<syn::Ident>() {
                FreeAttr::from_args(v, s).map(Self::Free)
            } else {
                // TODO: Improve error message
                panic!("Could not determine type of argument");
            }
        })
    }
}

#[derive(Default)]
pub struct OptionAttr {
    pub flags: Flags,
    pub parser: Option<Expr>,
    pub value: Option<Expr>,
    pub hidden: bool,
    pub help: Option<String>,
}

impl OptionAttr {
    fn from_args(first_flag: String, s: ParseStream) -> syn::Result<OptionAttr> {
        let mut option_attr = OptionAttr::default();
        option_attr.flags.add(&first_flag);

        parse_args(s, |s: ParseStream| {
            if let Ok(litstr) = s.parse::<LitStr>() {
                option_attr.flags.add(&litstr.value());
                return Ok(());
            }

            let ident = s.parse::<Ident>()?;
            match ident.to_string().as_ref() {
                "parser" => {
                    s.parse::<Token![=]>()?;
                    let p = s.parse::<Expr>()?;
                    option_attr.parser = Some(p);
                }
                "value" => {
                    s.parse::<Token![=]>()?;
                    let d = s.parse::<Expr>()?;
                    option_attr.value = Some(d);
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
                    ));
                }
            }
            Ok(())
        })?;

        Ok(option_attr)
    }
}

#[derive(Default)]
pub struct FreeAttr {
    pub filters: Vec<syn::Ident>,
}

impl FreeAttr {
    pub fn from_args(first_value: syn::Ident, s: ParseStream) -> syn::Result<Self> {
        let mut free_attr = FreeAttr::default();
        free_attr.filters.push(first_value);

        parse_args(s, |s: ParseStream| {
            let ident = s.parse::<Ident>()?;
            free_attr.filters.push(ident);
            Ok(())
        })?;

        Ok(free_attr)
    }
}

#[derive(Default)]
pub struct ValueAttr {
    pub keys: Vec<String>,
    pub value: Option<Expr>,
}

impl ValueAttr {
    pub fn parse(attr: &Attribute) -> syn::Result<Self> {
        let mut value_attr = Self::default();

        // value does not need to take arguments, so short circuit if it does not have one
        if let syn::Meta::Path(_) = &attr.meta {
            return Ok(value_attr);
        }

        attr.parse_args_with(|s: ParseStream| {
            loop {
                if let Ok(litstr) = s.parse::<LitStr>() {
                    value_attr.keys.push(litstr.value());
                } else {
                    let ident = s.parse::<Ident>()?;
                    match ident.to_string().as_str() {
                        "value" => {
                            s.parse::<Token![=]>()?;
                            let p = s.parse::<Expr>()?;
                            value_attr.value = Some(p);
                        }
                        _ => return Err(s.error("unrecognized keyword in value attribute")),
                    }
                }

                if s.is_empty() {
                    return Ok(());
                }
                s.parse::<Token![,]>()?;
                if s.is_empty() {
                    return Ok(());
                }
            }
        })?;

        Ok(value_attr)
    }
}

fn parse_args(
    s: ParseStream,
    mut logic: impl FnMut(ParseStream) -> syn::Result<()>,
) -> syn::Result<()> {
    loop {
        if s.is_empty() {
            return Ok(());
        }
        s.parse::<Token![,]>()?;
        if s.is_empty() {
            return Ok(());
        }
        logic(s)?;
    }
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
            ));
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
                ));
            }
        };
        strings.push(val);
    }
    Ok(strings)
}
