// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::{Command, ValueHint};

pub fn render(c: &Command) -> String {
    let mut out = String::new();
    let name = &c.name;
    for arg in &c.args {
        let mut line = format!("complete -c {name}");
        for short in &arg.short {
            line.push_str(&format!(" -s {short}"));
        }
        for long in &arg.long {
            line.push_str(&format!(" -l {long}"));
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
            let joined = s.join(", ");
            format!(" -a {{ {joined} }}")
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod test {
    use super::render;
    use crate::{Arg, Command};

    #[test]
    fn short() {
        let c = Command {
            name: "test".into(),
            args: vec![Arg {
                short: vec!["a".into()],
                help: "some flag".into(),
                ..Arg::default()
            }],
        };
        assert_eq!(render(&c), "complete -c test -s a -d 'some flag'\n",)
    }

    #[test]
    fn long() {
        let c = Command {
            name: "test".into(),
            args: vec![Arg {
                long: vec!["all".into()],
                help: "some flag".into(),
                ..Arg::default()
            }],
        };
        assert_eq!(render(&c), "complete -c test -l all -d 'some flag'\n",)
    }
}
