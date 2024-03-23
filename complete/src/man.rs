// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::{Command, Flag, Value};
use roff::{bold, italic, roman, Roff};

pub fn render(c: &Command) -> String {
    let mut page = Roff::new();
    page.control("TH", [&c.name.to_uppercase(), "1"]);
    page.control("SH", ["NAME"]);
    page.text([roman(c.name)]);
    page.control("SH", ["DESCRIPTION"]);
    page.text([roman(c.summary)]);
    page.control("SH", ["OPTIONS"]);

    for arg in &c.args {
        page.control("TP", []);

        let mut flags = Vec::new();
        for Flag { flag, value } in &arg.long {
            if !flags.is_empty() {
                flags.push(roman(", "));
            }
            flags.push(bold(format!("--{flag}")));
            match value {
                Value::Required(name) => {
                    flags.push(roman("="));
                    flags.push(italic(*name));
                }
                Value::Optional(name) => {
                    flags.push(roman("["));
                    flags.push(roman("="));
                    flags.push(italic(*name));
                    flags.push(roman("]"));
                }
                Value::No => {}
            }
        }
        for Flag { flag, value } in &arg.short {
            if !flags.is_empty() {
                flags.push(roman(", "));
            }
            flags.push(bold(format!("-{flag}")));
            match value {
                Value::Required(name) => {
                    flags.push(roman(" "));
                    flags.push(italic(*name));
                }
                Value::Optional(name) => {
                    flags.push(roman("["));
                    flags.push(italic(*name));
                    flags.push(roman("]"));
                }
                Value::No => {}
            }
        }
        for (flag, value) in &arg.dd_style {
            if !flags.is_empty() {
                flags.push(roman(", "));
            }
            flags.push(bold(*flag));
            flags.push(roman("="));
            flags.push(italic(*value));
        }
        page.text(flags);
        page.text([roman(arg.help)]);
    }

    page.control("SH", ["AUTHORS"]);
    page.text([roman(c.authors)]);

    page.control("SH", ["COPYRIGHT"]);
    page.text([roman(format!("Copyright © {}.", &c.authors))]);
    page.text([roman(format!("License: {}", &c.license))]);
    page.render()
}
