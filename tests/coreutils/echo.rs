use std::ffi::OsString;
use uutils_args::{Arguments, Options};

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
}

#[derive(Default)]
struct Settings {
    trailing_newline: bool,
    escape: bool,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::NoNewline => self.trailing_newline = false,
            Arg::EnableEscape => self.escape = true,
            Arg::DisableEscape => self.escape = false,
        }
        Ok(())
    }
}

// These next two tests exemplify echo style parsing. Which we have to
// support explicitly.

#[test]
#[ignore = "needs to be fixed after positional argument refactor"]
fn double_hyphen() {
    let (_, operands) = Settings::default().parse(["echo", "--"]).unwrap();
    assert_eq!(operands, vec![OsString::from("--")]);

    let (_, operands) = Settings::default().parse(["echo", "--", "-n"]).unwrap();
    assert_eq!(operands, vec![OsString::from("--"), OsString::from("-n")]);
}

#[test]
#[ignore]
fn nonexistent_options_are_values() {
    let (_, operands) = Settings::default().parse(["echo", "-f"]).unwrap();
    assert_eq!(operands, vec![OsString::from("-f")]);
}
