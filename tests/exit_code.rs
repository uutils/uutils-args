use uutils_args::Arguments;

#[test]
fn one_flag() {
    #[derive(Arguments, Clone, Debug, PartialEq, Eq)]
    #[arguments(exit_code = 4)]
    enum Arg {
        #[arg("-f", "--foo")]
        Foo,
    }

    assert_eq!(Arg::EXIT_CODE, 4);
}
