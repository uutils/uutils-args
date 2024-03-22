// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::{Command, Flag, ValueHint};

/// Create completion script for `fish`
///
/// Short and long options are combined into single `complete` calls, even if
/// they differ in whether they take arguments or not.
pub fn render(c: &Command) -> String {
    let mut out = String::new();
    let name = &c.name;
    for arg in &c.args {
        let mut line = format!("complete -c {name}");
        for Flag { flag, .. } in &arg.short {
            line.push_str(&format!(" -s {flag}"));
        }
        for Flag { flag, .. } in &arg.long {
            line.push_str(&format!(" -l {flag}"));
        }
        line.push_str(&format!(" -d '{}'", arg.help));
        if let Some(value) = &arg.value {
            line.push_str(&render_value_hint(value));
        }
        out.push_str(&line);
        out.push('\n');
    }
    out
}

fn render_value_hint(value: &ValueHint) -> String {
    match value {
        ValueHint::Strings(s) => {
            let joined = s.join(" ");
            format!(" -f -a \"{joined}\"")
        }
        ValueHint::AnyPath | ValueHint::FilePath | ValueHint::ExecutablePath => String::from(" -F"),
        ValueHint::DirPath => " -f -a \"(__fish_complete_directories)\"".into(),
        ValueHint::Unknown => " -f".into(),
        ValueHint::Username => " -f -a \"(__fish_complete_users)\"".into(),
        ValueHint::Hostname => " -f -a \"(__fish_print_hostnames)\"".into(),
    }
}

#[cfg(test)]
mod test {
    use super::render;
    use crate::{Arg, Command, Flag, Value, ValueHint};

    #[test]
    fn short() {
        let c = Command {
            name: "test",
            args: vec![Arg {
                short: vec![Flag {
                    flag: "a",
                    value: Value::No,
                }],
                help: "some flag",
                ..Arg::default()
            }],
            ..Command::default()
        };
        assert_eq!(render(&c), "complete -c test -s a -d 'some flag'\n",)
    }

    #[test]
    fn long() {
        let c = Command {
            name: "test",
            args: vec![Arg {
                long: vec![Flag {
                    flag: "all",
                    value: Value::No,
                }],
                help: "some flag",
                ..Arg::default()
            }],
            ..Command::default()
        };
        assert_eq!(render(&c), "complete -c test -l all -d 'some flag'\n",)
    }

    #[test]
    fn value_hints() {
        let args = [
            (
                ValueHint::Strings(vec!["all".into(), "none".into()]),
                "-f -a \"all none\"",
            ),
            (ValueHint::Unknown, "-f"),
            (ValueHint::AnyPath, "-F"),
            (ValueHint::FilePath, "-F"),
            (
                ValueHint::DirPath,
                "-f -a \"(__fish_complete_directories)\"",
            ),
            (ValueHint::ExecutablePath, "-F"),
            (ValueHint::Username, "-f -a \"(__fish_complete_users)\""),
            (ValueHint::Hostname, "-f -a \"(__fish_print_hostnames)\""),
        ];
        for (hint, expected) in args {
            let c = Command {
                name: "test",
                args: vec![Arg {
                    short: vec![Flag {
                        flag: "a",
                        value: Value::No,
                    }],
                    long: vec![],
                    dd_style: vec![],
                    help: "some flag",
                    value: Some(hint),
                }],
                ..Command::default()
            };
            assert_eq!(
                render(&c),
                format!("complete -c test -s a -d 'some flag' {expected}\n")
            )
        }
    }
}
