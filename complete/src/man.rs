use crate::Command;
use roff::{bold, roman, Roff};

pub fn render(c: &Command) -> String {
    let mut page = Roff::new();
    page.control("TH", [&c.name.to_uppercase(), "1"]);
    page.control("SH", ["NAME"]);
    page.text([roman(&c.name)]);
    page.control("SH", ["DESCRIPTION"]);
    page.text([roman(&c.summary)]);
    page.control("SH", ["OPTIONS"]);

    for arg in &c.args {
        page.control("TP", []);

        let mut flags = Vec::new();
        for l in &arg.long {
            if !flags.is_empty() {
                flags.push(roman(", "));
            }
            flags.push(bold(format!("--{l}")));
        }
        for s in &arg.short {
            if !flags.is_empty() {
                flags.push(roman(", "));
            }
            flags.push(bold(format!("-{s}")));
        }
        page.text(flags);
        page.text([roman(&arg.help)]);
    }
    page.render()
}
