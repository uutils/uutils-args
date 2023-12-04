use std::ffi::OsString;
use uutils_args::{Arguments, Initial, Options};

#[derive(Arguments)]
#[arguments(parse_echo_style)]
enum Arg {
    /// Do not output trailing newline
    #[arg("-n")]
    NoNewline,

    /// Enable interpretation of backslash escapes
    #[arg("-e")]
    EnableEscape,

    /// Disable interpretation of backslash escapes
    #[arg("-E")]
    DisableEscape,

    #[arg("STRING", last)]
    String(Vec<OsString>),
}

#[derive(Initial)]
struct Settings {
    trailing_newline: bool,
    escape: bool,
    strings: Vec<OsString>,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::NoNewline => self.trailing_newline = false,
            Arg::EnableEscape => self.escape = true,
            Arg::DisableEscape => self.escape = false,
            Arg::String(s) => self.strings = s,
        }
    }
}

// These next two tests exemplify echo style parsing. Which we have to
// support explicitly.

#[test]
fn double_hyphen() {
    let s = Settings::parse(["echo", "--"]);
    assert_eq!(s.strings, vec![OsString::from("--")]);

    let s = Settings::parse(["echo", "--", "-n"]);
    assert_eq!(s.strings, vec![OsString::from("--"), OsString::from("-n")]);
}

#[test]
#[ignore]
fn nonexistent_options_are_values() {
    let s = Settings::parse(["echo", "-f"]);
    assert_eq!(s.strings, vec![OsString::from("-f")]);
}
