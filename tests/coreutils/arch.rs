use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
#[help("--help")]
#[version("--version")]
enum Arg {}

#[derive(Default, Options)]
#[arg_type(Arg)]
struct Settings {}

#[test]
fn no_args() {
    assert!(Settings::parse(["arch"]).is_ok());
}

#[test]
fn one_arg_fails() {
    assert!(Settings::parse(["arch", "-f"]).is_err());
    assert!(Settings::parse(["arch", "--foo"]).is_err());
    assert!(Settings::parse(["arch", "foo"]).is_err());
}