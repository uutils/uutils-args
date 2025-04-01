// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::{Command, Flag};

/// Create completion script for `bash`
///
/// Short and long options are combined into single `complete` calls, even if
/// they differ in whether they take arguments or not; just like in case of `fish`.
/// Also, pretend that files are fine in any position. ValueHints are ignored entirely.
pub fn render(c: &Command) -> String {
    let mut out = String::new();
    // Be careful around the program '['!
    let name_identifier = if c.name == "[" { &"bracket" } else { &c.name };
    // Register _comp_uu_FOO as a bash function that computes completions:
    out.push_str(&format!(
        "complete -F _comp_uu_{name_identifier} '{}';",
        &c.name
    ));
    out.push_str(&format!("_comp_uu_{name_identifier}()"));
    // Unless the current argument starts with "-", pre-populate the completions list with all files and dirs:
    out.push_str("{ local cur;_init_completion||return;COMPREPLY=();if [[ \"$cur\" != \"-*\" ]]; then _filedir;fi;COMPREPLY+=($(compgen -W \"");
    for arg in &c.args {
        for Flag { flag, .. } in &arg.short {
            out.push_str(&format!("-{flag} "));
        }
        for Flag { flag, .. } in &arg.long {
            out.push_str(&format!("--{flag} "));
        }
    }
    out.push_str("\" -- \"$cur\"));}\n");
    out
}

#[cfg(test)]
mod test {
    use super::render;
    use crate::{Arg, Command, Flag, Value};

    #[test]
    fn simple() {
        let c = Command {
            name: "foo",
            args: vec![
                Arg {
                    short: vec![Flag {
                        flag: "a",
                        value: Value::No,
                    }],
                    long: vec![Flag {
                        flag: "all",
                        value: Value::No,
                    }],
                    ..Arg::default()
                },
                Arg {
                    short: vec![Flag {
                        flag: "x",
                        value: Value::No,
                    }],
                    ..Arg::default()
                },
            ],
            ..Command::default()
        };
        assert_eq!(render(&c), "complete -F _comp_uu_foo 'foo';_comp_uu_foo(){ local cur;_init_completion||return;COMPREPLY=();if [[ \"$cur\" != \"-*\" ]]; then _filedir;fi;COMPREPLY+=($(compgen -W \"-a --all -x \" -- \"$cur\"));}\n")
    }

    #[test]
    fn bracket() {
        let c = Command {
            name: "[",
            args: vec![Arg {
                short: vec![Flag {
                    flag: "x",
                    value: Value::No,
                }],
                ..Arg::default()
            }],
            ..Command::default()
        };
        assert_eq!(render(&c), "complete -F _comp_uu_bracket '[';_comp_uu_bracket(){ local cur;_init_completion||return;COMPREPLY=();if [[ \"$cur\" != \"-*\" ]]; then _filedir;fi;COMPREPLY+=($(compgen -W \"-x \" -- \"$cur\"));}\n")
    }
}
