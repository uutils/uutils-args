// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::{Arg, Command, Flag};

/// Create completion script for `zsh`
pub fn render(c: &Command) -> String {
    template(c.name, &render_args(&c.args))
}

fn render_args(args: &[Arg]) -> String {
    let mut out = String::new();
    let indent = " ".repeat(8);
    for arg in args {
        let help = &arg.help;
        for Flag { flag, .. } in &arg.short {
            out.push_str(&format!("{indent}'-{flag}[{help}]' \\\n"));
        }
        for Flag { flag, .. } in &arg.long {
            out.push_str(&format!("{indent}'--{flag}[{help}]' \\\n"));
        }
    }
    out
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

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext=\"$curcontext\" state line
    _arguments \"${{_arguments_options[@]}}\" \\\n{args}
&& ret=0
}}

if [ \"$funcstack[1]\" = \"_{name}\" ]; then
    {name} \"$@\"
else
    compdef _{name} {name}
fi"
    )
}
