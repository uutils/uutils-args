// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::{
    argument::{ArgType, Argument},
    flags::{Flag, Flags, Value},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn complete(args: &[Argument], file: &Option<String>) -> TokenStream {
    let mut arg_specs = Vec::new();

    let (summary, _usage, after_options) = if let Some(file) = file {
        crate::help::read_help_file(file)
    } else {
        ("".into(), "{} [OPTIONS] [ARGUMENTS]".into(), "".into())
    };

    for Argument {
        help,
        field,
        arg_type,
        ..
    } in args
    {
        let ArgType::Option {
            flags,
            hidden: false,
            ..
        } = arg_type
        else {
            continue;
        };

        let Flags {
            short,
            long,
            dd_style,
        } = flags;
        if short.is_empty() && long.is_empty() && dd_style.is_empty() {
            continue;
        }

        // If none of the flags take an argument, we won't need ValueHint
        // based on that type. So we should not attempt to call `value_hint`
        // on it.
        let any_flag_takes_argument = !dd_style.is_empty()
            && short.iter().any(|f| f.value != Value::No)
            && long.iter().any(|f| f.value != Value::No);

        let short: Vec<_> = short
            .iter()
            .map(|Flag { flag, value }| {
                let flag = flag.to_string();
                let value = match value {
                    Value::No => quote!(::uutils_args_complete::Value::No),
                    Value::Optional(name) => quote!(::uutils_args_complete::Value::Optional(#name)),
                    Value::Required(name) => quote!(::uutils_args_complete::Value::Required(#name)),
                };
                quote!(::uutils_args_complete::Flag {
                    flag: #flag,
                    value: #value
                })
            })
            .collect();

        let long: Vec<_> = long
            .iter()
            .map(|Flag { flag, value }| {
                let value = match value {
                    Value::No => quote!(::uutils_args_complete::Value::No),
                    Value::Optional(name) => quote!(::uutils_args_complete::Value::Optional(#name)),
                    Value::Required(name) => quote!(::uutils_args_complete::Value::Required(#name)),
                };
                quote!(::uutils_args_complete::Flag {
                    flag: #flag,
                    value: #value
                })
            })
            .collect();

        let dd_style: Vec<_> = dd_style
            .iter()
            .map(|(flag, value)| quote!((#flag, #value)))
            .collect();

        let hint = match (field, any_flag_takes_argument) {
            (Some(ty), true) => quote!(Some(<#ty>::value_hint())),
            _ => quote!(None),
        };

        arg_specs.push(quote!(
            ::uutils_args_complete::Arg {
                short: vec![#(#short),*],
                long: vec![#(#long),*],
                dd_style: vec![#(#dd_style),*],
                help: #help,
                value: #hint,
            }
        ))
    }

    quote!(::uutils_args_complete::Command {
        name: option_env!("CARGO_BIN_NAME").unwrap_or(env!("CARGO_PKG_NAME")),
        summary: #summary,
        after_options: #after_options,
        version: env!("CARGO_PKG_VERSION"),
        args: vec![#(#arg_specs),*],
        license: env!("CARGO_PKG_LICENSE"),
        authors: env!("CARGO_PKG_AUTHORS"),
    })
}
