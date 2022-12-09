use std::ops::RangeInclusive;

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, ExprLit, ExprRange, Ident, Lit, LitInt, LitStr, RangeLimits, Token,
};

use crate::{Arg, Expr};

#[derive(Default)]
pub(crate) struct FlagAttr {
    pub(crate) flags: Vec<Arg>,
    pub(crate) value: Option<syn::Expr>,
}

enum FlagAttrArg {
    Arg(Arg),
    Value(Expr),
}

#[derive(Default)]
pub(crate) struct OptionAttr {
    pub(crate) flags: Vec<Arg>,
    // This should probably not accept any expr to give better errors.
    // Closures should be allowed though.
    pub(crate) parser: Option<Expr>,
}

enum OptionAttrArg {
    Arg(Arg),
    Parser(Expr),
}

#[derive(Default)]
pub(crate) struct ValueAttr {
    pub(crate) keys: Vec<String>,
    pub(crate) value: Option<Expr>,
}

enum ValueAttrArg {
    Key(String),
    Value(Expr),
}

pub(crate) struct PositionalAttr {
    pub(crate) num_args: RangeInclusive<usize>,
}

impl Default for PositionalAttr {
    fn default() -> Self {
        Self { num_args: 1..=1 }
    }
}

enum PositionalAttrArg {
    NumArgs(RangeInclusive<usize>),
}

pub(crate) fn parse_flag_attr(attr: Attribute) -> FlagAttr {
    let mut flag_attr = FlagAttr::default();
    let Ok(parsed_args) = attr
        .parse_args_with(Punctuated::<FlagAttrArg, Token![,]>::parse_terminated)
    else {
        return flag_attr;
    };
    for arg in parsed_args {
        match arg {
            FlagAttrArg::Arg(a) => flag_attr.flags.push(a),
            FlagAttrArg::Value(e) => flag_attr.value = Some(e),
        };
    }
    flag_attr
}

impl Parse for FlagAttrArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            return parse_flag(input).map(Self::Arg);
        }

        if input.peek(Ident) {
            let name = input.parse::<Ident>()?.to_string();
            input.parse::<Token![=]>()?;
            match name.as_str() {
                "value" => return Ok(Self::Value(input.parse::<Expr>()?)),
                _ => panic!("Unrecognized argument {} for flag attribute", name),
            };
        }
        panic!("Arguments to flag attribute must be string literals");
    }
}

pub(crate) fn parse_option_attr(attr: Attribute) -> OptionAttr {
    let mut option_attr = OptionAttr::default();
    let Ok(parsed_args) = attr
        .parse_args_with(Punctuated::<OptionAttrArg, Token![,]>::parse_terminated)
    else {
        return option_attr;
    };

    for arg in parsed_args {
        match arg {
            OptionAttrArg::Arg(a) => option_attr.flags.push(a),
            OptionAttrArg::Parser(e) => option_attr.parser = Some(e),
        };
    }
    option_attr
}

impl Parse for OptionAttrArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            return parse_flag(input).map(Self::Arg);
        }

        if input.peek(Ident) {
            let name = input.parse::<Ident>()?.to_string();
            input.parse::<Token![=]>()?;
            match name.as_str() {
                "parser" => return Ok(Self::Parser(input.parse::<Expr>()?)),
                _ => panic!("Unrecognized argument {} for option attribute", name),
            };
        }
        panic!("Arguments to option attribute must be string literals");
    }
}

pub(crate) fn parse_value_attr(attr: Attribute) -> ValueAttr {
    let mut value_attr = ValueAttr::default();
    let Ok(parsed_args) = attr
        .parse_args_with(Punctuated::<ValueAttrArg, Token![,]>::parse_terminated)
    else {
        return value_attr;
    };

    for arg in parsed_args {
        match arg {
            ValueAttrArg::Key(k) => value_attr.keys.push(k),
            ValueAttrArg::Value(e) => value_attr.value = Some(e),
        };
    }

    value_attr
}

impl Parse for ValueAttrArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            return Ok(Self::Key(input.parse::<LitStr>()?.value()));
        }

        if input.peek(Ident) {
            let name = input.parse::<Ident>()?.to_string();
            input.parse::<Token![=]>()?;
            match name.as_str() {
                "value" => return Ok(Self::Value(input.parse::<Expr>()?)),
                _ => panic!("Unrecognized argument {} for option attribute", name),
            };
        }
        panic!("Arguments to option attribute must be string literals");
    }
}

fn parse_flag(input: ParseStream) -> syn::Result<Arg> {
    let str = input.parse::<LitStr>().unwrap().value();
    if let Some(s) = str.strip_prefix("--") {
        return Ok(Arg::Long(s.to_owned()));
    } else if let Some(s) = str.strip_prefix('-') {
        assert_eq!(
            s.len(),
            1,
            "Exactly one character must follow '-' in a flag attribute"
        );
        return Ok(Arg::Short(s.chars().next().unwrap()));
    }
    panic!("Arguments to flag must start with \"-\" or \"--\"");
}

pub(crate) fn parse_positional_attr(attr: Attribute) -> PositionalAttr {
    let mut positional_attr = PositionalAttr::default();
    let Ok(parsed_args) = attr
        .parse_args_with(Punctuated::<PositionalAttrArg, Token![,]>::parse_terminated)
    else {
        return positional_attr;
    };

    for arg in parsed_args {
        match arg {
            PositionalAttrArg::NumArgs(k) => positional_attr.num_args = k,
        };
    }

    positional_attr
}

impl Parse for PositionalAttrArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if (input.peek(LitInt) && input.peek2(Token![..])) || input.peek(Token![..]) {
            // We're dealing with a range
            let range = input.parse::<ExprRange>()?;
            let from = match range.from.as_deref() {
                Some(Expr::Lit(ExprLit {
                    lit: Lit::Int(i), ..
                })) => i.base10_parse::<usize>().unwrap(),
                None => 0,
                _ => panic!("Range must consist of usize"),
            };

            let inclusive = matches!(range.limits, RangeLimits::Closed(_));
            let to = match range.to.as_deref() {
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

            return Ok(Self::NumArgs(match to {
                Some(to) => from..=to,
                None => from..=usize::MAX,
            }));
        } else if input.peek(LitInt) {
            // We're dealing with a single interger
            let int = input.parse::<LitInt>()?;
            let suffix = int.suffix();
            assert!(
                suffix == "" || suffix == "usize",
                "The position index must be usize"
            );
            let n = int.base10_parse::<usize>().unwrap();
            return Ok(Self::NumArgs(n..=n));
        }

        // if input.peek(Ident) {
        //     let name = input.parse::<Ident>()?.to_string();
        //     input.parse::<Token![=]>()?;
        //     match name.as_str() {
        //         "value" => return Ok(Self::Value(input.parse::<Expr>()?)),
        //         _ => panic!("Unrecognized argument {} for option attribute", name),
        //     };
        // }
        panic!("unpexpected argument to positional");
    }
}
