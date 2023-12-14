use uutils_args::Arguments;

#[derive(Arguments)]
enum Arg {}

#[test]
fn no_args() {
    assert!(Arg::try_check(["arch"]).is_ok());
}

#[test]
fn one_arg_fails() {
    assert!(Arg::try_check(["arch", "-f"]).is_err());
    assert!(Arg::try_check(["arch", "--foo"]).is_err());
}
