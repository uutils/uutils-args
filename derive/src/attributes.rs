use std::ops::RangeInclusive;

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Expr, ExprLit, ExprRange, Ident, Lit, LitInt, LitStr, RangeLimits, Token,
};

use crate::flags::Flags;

pub(crate) enum ArgAttr {
    Option(OptionAttr),
    Positional(PositionalAttr),
}

pub(crate) fn parse_argument_attribute(attr: &Attribute) -> ArgAttr {
    if attr.path.is_ident("option") {
        ArgAttr::Option(OptionAttr::parse(attr))
    } else if attr.path.is_ident("positional") {
        ArgAttr::Positional(PositionalAttr::parse(attr))
    } else {
        panic!("Internal error: invalid argument attribute");
    }
}

enum AttributeArguments {
    String(String),
    Parser(Expr),
    Default(Expr),
    Value(Expr),
    NumArgs(RangeInclusive<usize>),
    File(String),
    Env(String),
    ExitCode(i32),
    Help(String),
    HelpFlags(Vec<String>),
    VersionFlags(Vec<String>),
    Last,
    Hidden,
}

impl AttributeArguments {
    fn parse_all(attr: &Attribute) -> Vec<Self> {
        attr.parse_args_with(Punctuated::<AttributeArguments, Token![,]>::parse_terminated)
            .map(|iter| iter.into_iter().collect::<Vec<_>>())
            .unwrap_or_default()
    }
}

pub(crate) struct ArgumentsAttr {
    pub(crate) help_flags: Flags,
    pub(crate) version_flags: Flags,
    pub(crate) file: Option<String>,
    pub(crate) exit_code: i32,
}

impl Default for ArgumentsAttr {
    fn default() -> Self {
        Self {
            help_flags: Flags::new(["--help"]),
            version_flags: Flags::new(["--version"]),
            file: None,
            exit_code: 1,
        }
    }
}

impl ArgumentsAttr {
    pub(crate) fn parse(attr: &Attribute) -> Self {
        let mut arguments_attr = Self::default();
        for arg in AttributeArguments::parse_all(attr) {
            match arg {
                AttributeArguments::HelpFlags(flags) => {
                    arguments_attr.help_flags = Flags::new(flags);
                }
                AttributeArguments::VersionFlags(flags) => {
                    arguments_attr.version_flags = Flags::new(flags);
                }
                AttributeArguments::File(s) => arguments_attr.file = Some(s),
                AttributeArguments::ExitCode(code) => arguments_attr.exit_code = code,
                _ => panic!(),
            }
        }

        arguments_attr
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
    pub(crate) fn parse(attr: &Attribute) -> Self {
        let mut option_attr = OptionAttr::default();

        for arg in AttributeArguments::parse_all(attr) {
            match arg {
                AttributeArguments::String(a) => option_attr.flags.add(&a),
                AttributeArguments::Parser(e) => option_attr.parser = Some(e),
                AttributeArguments::Default(e) => option_attr.default = Some(e),
                AttributeArguments::Hidden => option_attr.hidden = true,
                AttributeArguments::Help(h) => option_attr.help = Some(h),
                _ => panic!("Invalid argument"),
            };
        }

        assert!(
            !option_attr.flags.is_empty(),
            "must give a flag in an option attribute"
        );

        option_attr
    }
}

#[derive(Default)]
pub(crate) struct FieldAttr {
    pub(crate) default: Option<Expr>,
    pub(crate) env: Option<String>,
}

impl FieldAttr {
    pub(crate) fn parse(attr: &Attribute) -> Self {
        let mut field_attr = Self::default();

        for arg in AttributeArguments::parse_all(attr) {
            match arg {
                AttributeArguments::Default(e) => field_attr.default = Some(e),
                AttributeArguments::Env(e) => field_attr.env = Some(e),
                _ => panic!("Invalid argument"),
            };
        }

        field_attr
    }
}

#[derive(Default)]
pub(crate) struct ValueAttr {
    pub(crate) keys: Vec<String>,
    pub(crate) value: Option<Expr>,
}

impl ValueAttr {
    pub(crate) fn parse(attr: &Attribute) -> Self {
        let mut value_attr = Self::default();

        for arg in AttributeArguments::parse_all(attr) {
            match arg {
                AttributeArguments::String(k) => value_attr.keys.push(k),
                AttributeArguments::Value(e) => value_attr.value = Some(e),
                _ => panic!(),
            };
        }

        value_attr
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
    pub(crate) fn parse(attr: &Attribute) -> Self {
        let mut positional_attr = Self::default();

        for arg in AttributeArguments::parse_all(attr) {
            match arg {
                AttributeArguments::NumArgs(k) => positional_attr.num_args = k,
                AttributeArguments::Last => positional_attr.last = true,
                _ => panic!(),
            };
        }

        positional_attr
    }
}

impl Parse for AttributeArguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            return Ok(Self::String(input.parse::<LitStr>().unwrap().value()));
        }

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
                suffix.is_empty() || suffix == "usize",
                "The position index must be usize"
            );
            let n = int.base10_parse::<usize>().unwrap();
            return Ok(Self::NumArgs(n..=n));
        }

        if input.peek(Ident) {
            let name = input.parse::<Ident>()?.to_string();

            // Arguments that do not take values
            match name.as_str() {
                "last" => return Ok(Self::Last),
                "hidden" => return Ok(Self::Hidden),
                _ => {}
            };

            input.parse::<Token![=]>()?;

            // Arguments that do take values
            match name.as_str() {
                "parser" => return Ok(Self::Parser(input.parse::<Expr>()?)),
                "default" => return Ok(Self::Default(input.parse::<Expr>()?)),
                "value" => return Ok(Self::Value(input.parse::<Expr>()?)),
                "file" => return Ok(Self::File(input.parse::<LitStr>()?.value())),
                "env" => return Ok(Self::Env(input.parse::<LitStr>()?.value())),
                "help" => return Ok(Self::Help(input.parse::<LitStr>()?.value())),
                "exit_code" => return Ok(Self::ExitCode(input.parse::<LitInt>()?.base10_parse()?)),
                "help_flags" => {
                    let expr = input.parse::<Expr>()?;
                    let arr = match expr {
                        syn::Expr::Array(arr) => arr,
                        _ => panic!("Argument to `help` must be an array"),
                    };

                    let mut strings = Vec::new();
                    for elem in arr.elems {
                        let val = match elem {
                            syn::Expr::Lit(syn::ExprLit {
                                attrs: _,
                                lit: syn::Lit::Str(litstr),
                            }) => litstr.value(),
                            _ => panic!("Argument to `help` must be an array of string literals"),
                        };
                        strings.push(val);
                    }
                    return Ok(Self::HelpFlags(strings));
                }
                "version_flags" => {
                    let expr = input.parse::<Expr>()?;
                    let arr = match expr {
                        syn::Expr::Array(arr) => arr,
                        _ => panic!("Argument to `version` must be an array"),
                    };

                    let mut strings = Vec::new();
                    for elem in arr.elems {
                        let val = match elem {
                            syn::Expr::Lit(syn::ExprLit {
                                attrs: _,
                                lit: syn::Lit::Str(litstr),
                            }) => litstr.value(),
                            _ => {
                                panic!("Argument to `version` must be an array of string literals")
                            }
                        };
                        strings.push(val);
                    }
                    return Ok(Self::VersionFlags(strings));
                }
                _ => panic!("Unrecognized argument {} for option attribute", name),
            };
        }
        panic!("Arguments to option attribute must be string literals");
    }
}
