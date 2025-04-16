// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::complete::{Arg, Command, Flag, Value, ValueHint};
use std::fmt::Write;

/// Create completion script for `nushell`
pub fn render(c: &Command) -> String {
    let mut args = Vec::new();
    let command_name = c.name;
    let mut complete_commands = Vec::new();
    let indent = " ".repeat(4);

    for arg in &c.args {
        let hint = if let Some((cmd, hint_name)) = render_completion_command(command_name, arg) {
            complete_commands.push(cmd);
            hint_name
        } else {
            "".into()
        };

        for Flag { flag, value } in &arg.short {
            let value = if let Value::Required(_) | Value::Optional(_) = value {
                format!(": string{hint}")
            } else {
                "".into()
            };
            args.push((format!("-{flag}{value}"), arg.help));
        }
        for Flag { flag, value } in &arg.long {
            let value = if let Value::Required(_) | Value::Optional(_) = value {
                format!(": string{hint}")
            } else {
                "".into()
            };
            args.push((format!("--{flag}{value}"), arg.help));
        }
    }
    let longest_arg = args.iter().map(|a| a.0.len()).max().unwrap_or_default();
    let mut arg_str = String::new();
    for (a, h) in args {
        writeln!(arg_str, "{indent}{a:<longest_arg$} # {h}").unwrap();
    }
    template(c.name, &complete_commands.join("\n"), &arg_str)
}

fn render_completion_command(command_name: &str, arg: &Arg) -> Option<(String, String)> {
    let val = arg.value.as_ref()?;

    // It could be that there is only a `dd` style argument. In that case, nu won't support it;
    let arg_name = arg.long.first().or(arg.short.first())?.flag;

    render_value_hint(val).map(|hint| {
        let name = format!("nu-complete {command_name} {arg_name}");
        let cmd = format!("def \"{name}\" [] {{\n    {hint}\n}}");
        let hint_str = format!("@\"{name}\"");
        (cmd, hint_str)
    })
}

fn render_value_hint(value: &ValueHint) -> Option<String> {
    match value {
        ValueHint::Strings(s) => {
            let vals = s
                .iter()
                .map(|s| format!("\"{s}\""))
                .collect::<Vec<_>>()
                .join(", ");
            Some(format!("[{vals}]"))
        }
        // The path arguments could be improved, but nu currently does not give
        // us enough context to improve the default completions.
        ValueHint::Unknown
        | ValueHint::AnyPath
        | ValueHint::FilePath
        | ValueHint::ExecutablePath
        | ValueHint::DirPath
        | ValueHint::Username
        | ValueHint::Hostname => None,
    }
}

fn template(name: &str, complete_commands: &str, args: &str) -> String {
    format!("{complete_commands}\n\nexport extern \"{name}\" [\n{args}]\n")
}
