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
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
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
        Ok(())
    }
}

#[test]
fn binary() {
    assert!(!Settings::default().parse(["b2sum"]).unwrap().0.binary);
    assert!(
        !Settings::default()
            .parse(["b2sum", "--text"])
            .unwrap()
            .0
            .binary
    );
    assert!(!Settings::default().parse(["b2sum", "-t"]).unwrap().0.binary);
    assert!(
        !Settings::default()
            .parse(["b2sum", "--binary", "--text"])
            .unwrap()
            .0
            .binary
    );
    assert!(
        !Settings::default()
            .parse(["b2sum", "-b", "-t"])
            .unwrap()
            .0
            .binary
    );

    assert!(
        Settings::default()
            .parse(["b2sum", "--binary"])
            .unwrap()
            .0
            .binary
    );
    assert!(Settings::default().parse(["b2sum", "-b"]).unwrap().0.binary);
    assert!(
        Settings::default()
            .parse(["b2sum", "--text", "--binary"])
            .unwrap()
            .0
            .binary
    );
    assert!(
        Settings::default()
            .parse(["b2sum", "-t", "-b"])
            .unwrap()
            .0
            .binary
    );
}

#[test]
fn check_output() {
    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--warn"])
            .unwrap()
            .0
            .check_output,
        CheckOutput::Warn
    );
    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--quiet"])
            .unwrap()
            .0
            .check_output,
        CheckOutput::Quiet
    );
    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--status"])
            .unwrap()
            .0
            .check_output,
        CheckOutput::Status
    );
    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--status", "--warn"])
            .unwrap()
            .0
            .check_output,
        CheckOutput::Warn
    );
    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--status", "--warn"])
            .unwrap()
            .0
            .check_output,
        CheckOutput::Warn
    );

    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--warn", "--quiet"])
            .unwrap()
            .0
            .check_output,
        CheckOutput::Quiet
    );

    assert_eq!(
        Settings::default()
            .parse(["b2sum", "--quiet", "--status"])
            .unwrap()
            .0
            .check_output,
        CheckOutput::Status
    );
}

#[test]
fn files() {
    assert_eq!(
        Settings::default()
            .parse(["b2sum", "foo", "bar"])
            .unwrap()
            .1,
        vec![OsString::from("foo"), OsString::from("bar")]
    );
}
