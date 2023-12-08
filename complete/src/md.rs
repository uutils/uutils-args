// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::Command;

/// Render command to a markdown file for mdbook
pub fn render(c: &Command) -> String {
    let mut out = String::new();
    out.push_str(&title(c));
    out.push_str(&additional(c));
    out.push_str(&c.summary);
    out.push_str("\n\n");
    out.push_str(&options(c));
    out.push_str("\n\n");
    out.push_str(&c.after_options);
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

        for long in &arg.long {
            flags.push(format!("<code>--{long}</code>"));
        }

        for short in &arg.short {
            flags.push(format!("<code>-{short}</code>"))
        }

        out.push_str(&flags.join(", "));
        out.push_str("</dt>\n");
        out.push_str(&format!("<dd>\n\n{}\n\n</dd>\n", arg.help));
    }
    out.push_str("</dl>");
    out
}
