// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::{
    argument::{ArgType, Argument},
    flags::{Flag, Flags},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn complete(args: &[Argument]) -> TokenStream {
    let mut arg_specs = Vec::new();

    for Argument { help, arg_type, .. } in args {
        let ArgType::Option {
            flags,
            hidden: false,
            ..
        } = arg_type
        else {
            continue;
        };

        let Flags { short, long, .. } = flags;
        if short.is_empty() && long.is_empty() {
            continue;
        }
        let short: Vec<_> = short
            .iter()
            .map(|Flag { flag, .. }| quote!(String::from(#flag)))
            .collect();
        let long: Vec<_> = long
            .iter()
            .map(|Flag { flag, .. }| quote!(String::from(#flag)))
            .collect();

        arg_specs.push(quote!(
            Arg {
                short: vec![#(#short),*],
                long: vec![#(#long),*],
                help: String::from(#help),
                value: None,
            }
        ))
    }

    quote!(Command {
        name: String::from(option_env!("CARGO_BIN_NAME").unwrap_or(env!("CARGO_PKG_NAME"))),
        args: vec![#(#arg_specs),*]
    })
}
