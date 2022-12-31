use std::path::{Path, PathBuf};

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
