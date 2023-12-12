// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::{Arg, Command, Flag, Value};
use std::fmt::Write;

/// Create completion script for `nushell`
pub fn render(c: &Command) -> String {
    let mut args = Vec::new();
    let indent = " ".repeat(4);

    for Arg {
        short,
        long,
        help,
        value: _value,
    } in &c.args
    {
        for Flag { flag, value } in short {
            let value = if let Value::Required(_) | Value::Optional(_) = value {
                ": string"
            } else {
                ""
            };
            args.push((format!("-{flag}{value}"), help));
        }
        for Flag { flag, value } in long {
            let value = if let Value::Required(_) | Value::Optional(_) = value {
                ": string"
            } else {
                ""
            };
            args.push((format!("--{flag}{value}"), help));
        }
    }
    let longest_arg = args.iter().map(|a| a.0.len()).max().unwrap_or_default();
    let mut arg_str = String::new();
    for (a, h) in args {
        writeln!(arg_str, "{indent}{a:<longest_arg$} # {h}").unwrap();
    }
    template(c.name, &arg_str)
}

fn template(name: &str, args: &str) -> String {
    format!(
        "\
        export extern {name} [\n{args}\
        ]\n\
        "
    )
}
