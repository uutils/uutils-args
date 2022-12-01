use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Ident, LitStr, Token,
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
pub(crate) struct MapAttr {
    pub(crate) arms: Vec<syn::Arm>,
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

pub(crate) fn parse_map(attr: Attribute) -> Option<MapAttr> {
    if !attr.path.is_ident("map") {
        return None;
    }

    let parsed_args = attr
        .parse_args_with(Punctuated::<syn::Arm, syn::parse::Nothing>::parse_terminated)
        .expect("Arguments to map must be valid match arms");

    Some(MapAttr {
        arms: parsed_args.into_iter().collect(),
    })
}
