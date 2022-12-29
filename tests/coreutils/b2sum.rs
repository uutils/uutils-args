use std::path::{PathBuf, Path};

use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
#[help("--help")]
#[version("--version")]
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

#[derive(Default, Options)]
#[arg_type(Arg)]
struct Settings {
    #[map(
        Arg::Binary => true,
        Arg::Text => false,
    )]
    binary: bool,

    #[map(Arg::Check => true)]
    check: bool,

    #[map(Arg::Tag => true)]
    tag: bool,

    #[map(
        Arg::Warn => CheckOutput::Warn,
        Arg::Quiet => CheckOutput::Quiet,
        Arg::Status => CheckOutput::Status,
    )]
    check_output: CheckOutput,

    #[map(Arg::Strict => true)]
    strict: bool,

    #[collect(set(Arg::File))]
    files: Vec<PathBuf>,
}

#[test]
fn binary() {
    assert!(!Settings::parse(["b2sum"]).unwrap().binary);
    assert!(!Settings::parse(["b2sum", "--text"]).unwrap().binary);
    assert!(!Settings::parse(["b2sum", "-t"]).unwrap().binary);
    assert!(
        !Settings::parse(["b2sum", "--binary", "--text"])
            .unwrap()
            .binary
    );
    assert!(!Settings::parse(["b2sum", "-b", "-t"]).unwrap().binary);

    assert!(Settings::parse(["b2sum", "--binary"]).unwrap().binary);
    assert!(Settings::parse(["b2sum", "-b"]).unwrap().binary);
    assert!(
        Settings::parse(["b2sum", "--text", "--binary"])
            .unwrap()
            .binary
    );
    assert!(Settings::parse(["b2sum", "-t", "-b"]).unwrap().binary);
}

#[test]
fn check_output() {
    assert_eq!(
        Settings::parse(["b2sum", "--warn"]).unwrap().check_output,
        CheckOutput::Warn
    );
    assert_eq!(
        Settings::parse(["b2sum", "--quiet"]).unwrap().check_output,
        CheckOutput::Quiet
    );
    assert_eq!(
        Settings::parse(["b2sum", "--status"]).unwrap().check_output,
        CheckOutput::Status
    );
    assert_eq!(
        Settings::parse(["b2sum", "--status", "--warn"])
            .unwrap()
            .check_output,
        CheckOutput::Warn
    );
    assert_eq!(
        Settings::parse(["b2sum", "--status", "--warn"])
            .unwrap()
            .check_output,
        CheckOutput::Warn
    );

    assert_eq!(
        Settings::parse(["b2sum", "--warn", "--quiet"])
            .unwrap()
            .check_output,
        CheckOutput::Quiet
    );

    assert_eq!(
        Settings::parse(["b2sum", "--quiet", "--status"])
            .unwrap()
            .check_output,
        CheckOutput::Status
    );
}

#[test]
fn files() {
    assert_eq!(
        Settings::parse(["b2sum", "foo", "bar"])
            .unwrap()
            .files,
        vec![Path::new("foo"), Path::new("bar")]
    );
}
