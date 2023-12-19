use std::ffi::OsString;
use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
enum Arg {
    #[arg("-b", "--binary")]
    Binary,

    #[arg("-c", "--check")]
    Check,

    #[arg("--tag")]
    Tag,

    #[arg("-t", "--text")]
    Text,

    #[arg("-q", "--quiet")]
    Quiet,

    #[arg("-s", "--status")]
    Status,

    #[arg("--strict")]
    Strict,

    #[arg("-w", "--warn")]
    Warn,
}

#[derive(Default, Debug, PartialEq, Eq)]
enum CheckOutput {
    #[default]
    Warn,
    Quiet,
    Status,
}

#[derive(Default)]
struct Settings {
    binary: bool,
    check: bool,
    tag: bool,
    check_output: CheckOutput,
    strict: bool,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::Binary => self.binary = true,
            Arg::Check => self.check = true,
            Arg::Tag => self.tag = true,
            Arg::Text => self.binary = false,
            Arg::Quiet => self.check_output = CheckOutput::Quiet,
            Arg::Status => self.check_output = CheckOutput::Status,
            Arg::Strict => self.strict = true,
            Arg::Warn => self.check_output = CheckOutput::Warn,
        }
    }
}

#[test]
fn binary() {
    assert!(!Settings::default().parse(["b2sum"]).0.binary);
    assert!(!Settings::default().parse(["b2sum", "--text"]).0.binary);
    assert!(!Settings::default().parse(["b2sum", "-t"]).0.binary);
    assert!(
        !Settings::default()
            .parse(["b2sum", "--binary", "--text"])
            .0
            .binary
    );
    assert!(!Settings::default().parse(["b2sum", "-b", "-t"]).0.binary);

    assert!(Settings::default().parse(["b2sum", "--binary"]).0.binary);
    assert!(Settings::default().parse(["b2sum", "-b"]).0.binary);
    assert!(
        Settings::default()
            .parse(["b2sum", "--text", "--binary"])
            .0
            .binary
    );
    assert!(Settings::default().parse(["b2sum", "-t", "-b"]).0.binary);
}

#[test]
fn check_output() {
    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--warn"])
            .0
            .check_output,
        CheckOutput::Warn
    );
    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--quiet"])
            .0
            .check_output,
        CheckOutput::Quiet
    );
    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--status"])
            .0
            .check_output,
        CheckOutput::Status
    );
    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--status", "--warn"])
            .0
            .check_output,
        CheckOutput::Warn
    );
    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--status", "--warn"])
            .0
            .check_output,
        CheckOutput::Warn
    );

    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--warn", "--quiet"])
            .0
            .check_output,
        CheckOutput::Quiet
    );

    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--quiet", "--status"])
            .0
            .check_output,
        CheckOutput::Status
    );
}

#[test]
fn files() {
    assert_eq!(
        Settings::default().parse(["b2sum", "foo", "bar"]).1,
        vec![OsString::from("foo"), OsString::from("bar")]
    );
}
