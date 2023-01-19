use std::path::{Path, PathBuf};

use uutils_args::{Arguments, Initial, Options};

#[derive(Clone, Arguments)]
enum Arg {
    #[option("-b", "--binary")]
    Binary,

    #[option("-c", "--check")]
    Check,

    #[option("--tag")]
    Tag,

    #[option("-t", "--text")]
    Text,

    #[option("-q", "--quiet")]
    Quiet,

    #[option("-s", "--status")]
    Status,

    #[option("--strict")]
    Strict,

    #[option("-w", "--warn")]
    Warn,

    #[positional(..)]
    File(PathBuf),
}

#[derive(Default, Debug, PartialEq, Eq)]
enum CheckOutput {
    #[default]
    Warn,
    Quiet,
    Status,
}

#[derive(Initial)]
struct Settings {
    binary: bool,
    check: bool,
    tag: bool,
    check_output: CheckOutput,
    strict: bool,
    files: Vec<PathBuf>,
}

impl Options for Settings {
    type Arg = Arg;
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
            Arg::File(f) => self.files.push(f),
        }
    }
}

#[test]
fn binary() {
    assert!(!Settings::parse(["b2sum"]).binary);
    assert!(!Settings::parse(["b2sum", "--text"]).binary);
    assert!(!Settings::parse(["b2sum", "-t"]).binary);
    assert!(!Settings::parse(["b2sum", "--binary", "--text"]).binary);
    assert!(!Settings::parse(["b2sum", "-b", "-t"]).binary);

    assert!(Settings::parse(["b2sum", "--binary"]).binary);
    assert!(Settings::parse(["b2sum", "-b"]).binary);
    assert!(Settings::parse(["b2sum", "--text", "--binary"]).binary);
    assert!(Settings::parse(["b2sum", "-t", "-b"]).binary);
}

#[test]
fn check_output() {
    assert_eq!(
        Settings::parse(["b2sum", "--warn"]).check_output,
        CheckOutput::Warn
    );
    assert_eq!(
        Settings::parse(["b2sum", "--quiet"]).check_output,
        CheckOutput::Quiet
    );
    assert_eq!(
        Settings::parse(["b2sum", "--status"]).check_output,
        CheckOutput::Status
    );
    assert_eq!(
        Settings::parse(["b2sum", "--status", "--warn"]).check_output,
        CheckOutput::Warn
    );
    assert_eq!(
        Settings::parse(["b2sum", "--status", "--warn"]).check_output,
        CheckOutput::Warn
    );

    assert_eq!(
        Settings::parse(["b2sum", "--warn", "--quiet"]).check_output,
        CheckOutput::Quiet
    );

    assert_eq!(
        Settings::parse(["b2sum", "--quiet", "--status"]).check_output,
        CheckOutput::Status
    );
}

#[test]
fn files() {
    assert_eq!(
        Settings::parse(["b2sum", "foo", "bar"]).files,
        vec![Path::new("foo"), Path::new("bar")]
    );
}
