// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::complete::{Arg, Command, Flag, Value, ValueHint};

/// Create completion script for `zsh`
pub fn render(c: &Command) -> String {
    template(c.name, &render_args(&c.args))
}

fn render_args(args: &[Arg]) -> String {
    let mut out = String::new();
    let indent = " ".repeat(8);

    // The reference for this can be found here:
    // https://zsh.sourceforge.io/Doc/Release/Completion-System.html#Completion-System
    for arg in args {
        let help = &arg.help;
        let hint = arg
            .value
            .as_ref()
            .map(render_value_hint)
            .unwrap_or_default();
        for Flag { flag, value } in &arg.short {
            let s = match value {
                // No special specifier, so there might be a space in-between the flag and argument.
                // The single colon means it's a required argument.
                Value::Required(name) => format!("-{flag}[{help}]:{name}:{hint}"),
                // '-' means that there can be no space in-between the flag and the argument
                // The double colon means it's an optional argument.
                Value::Optional(name) => format!("-{flag}-[{help}]::{name}:{hint}"),
                Value::No => format!("-{flag}[{help}]"),
            };
            out.push_str(&format!("{indent}'{s}'\\\n"));
        }
        for Flag { flag, value } in &arg.long {
            let s = match value {
                // '=' means either `=` or space in-between flag and argument.
                // The single colon means it's a required argument.
                Value::Required(name) => format!("--{flag}=[{help}]:{name}:{hint}"),
                // '=-' means that there must be a `=` for the argument.
                // The double colon means it's an optional argument.
                Value::Optional(name) => format!("--{flag}=-[{help}]::{name}:{hint}"),
                Value::No => format!("--{flag}[{help}]"),
            };
            out.push_str(&format!("{indent}'{s}' \\\n"));
        }
    }
    out
}

fn render_value_hint(value: &ValueHint) -> String {
    match value {
        ValueHint::Strings(s) => {
            let joined = s.join(" ");
            format!("({joined})")
        }
        ValueHint::Unknown => "".into(),
        ValueHint::AnyPath | ValueHint::FilePath => "_files".into(),
        ValueHint::ExecutablePath => "_absolute_command_paths".into(),
        ValueHint::DirPath => "_directories".into(),
        ValueHint::Username => "_users".into(),
        ValueHint::Hostname => "_hosts".into(),
    }
}

fn template(name: &str, args: &str) -> String {
    format!(
        "\
#compdef {name}

autoload -U is-at-least

_{name}() {{
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    # -s: enable option stacking
    # -S: Do not complete options after a '--' appearing on the line, and ignore the '--'
    # -C: Modify the curcontext parameter for an action of the form '->state'
    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext=\"$curcontext\" state line
    _arguments \"${{_arguments_options[@]}}\" \\\n{args}    && ret=0
}}

if [ \"$funcstack[1]\" = \"_{name}\" ]; then
    {name} \"$@\"
else
    compdef _{name} {name}
fi"
    )
}
