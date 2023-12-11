// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::{Command, Flag, Value};

/// Render command to a markdown file for mdbook
pub fn render(c: &Command) -> String {
    let mut out = String::new();
    out.push_str(&title(c));
    out.push_str(&additional(c));
    out.push_str(c.summary);
    out.push_str("\n\n");
    out.push_str(&options(c));
    out.push_str("\n\n");
    out.push_str(c.after_options);
    out.push('\n');
    out
}

fn title(c: &Command) -> String {
    format!("# {}\n\n", c.name)
}

fn additional(c: &Command) -> String {
    let version = &c.version;
    format!(
        "\
        <div class=\"additional\">\
        {version}\
        </div>\n\n\
        "
    )
}

fn options(c: &Command) -> String {
    let mut out = String::from("## Options\n\n");
    out.push_str("<dl>\n");
    for arg in &c.args {
        out.push_str("<dt>");

        let mut flags = Vec::new();

        for Flag { flag, value } in &arg.long {
            let value_str = match value {
                Value::Required(name) => format!("={name}"),
                Value::Optional(name) => format!("[={name}]"),
                Value::No => String::new(),
            };
            flags.push(format!("<code>--{flag}{value_str}</code>"));
        }

        for Flag { flag, value } in &arg.short {
            let value_str = match value {
                Value::Required(name) => format!(" {name}"),
                Value::Optional(name) => format!("[{name}]"),
                Value::No => String::new(),
            };
            flags.push(format!("<code>-{flag}{value_str}</code>"));
        }

        out.push_str(&flags.join(", "));
        out.push_str("</dt>\n");
        out.push_str(&format!("<dd>\n\n{}\n\n</dd>\n", arg.help));
    }
    out.push_str("</dl>");
    out
}
