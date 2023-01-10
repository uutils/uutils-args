use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
enum Arg {}

#[derive(Default, Options)]
#[arg_type(Arg)]
struct Settings {}

#[test]
fn no_args() {
    assert!(Settings::try_parse(["arch"]).is_ok());
}

#[test]
fn one_arg_fails() {
    assert!(Settings::try_parse(["arch", "-f"]).is_err());
    assert!(Settings::try_parse(["arch", "--foo"]).is_err());
    assert!(Settings::try_parse(["arch", "foo"]).is_err());
}
