use uutils_args::Options;

#[test]
fn one_flag() {
    #[derive(Default, Options)]
    struct Settings {
        #[flag]
        a: bool,
    }

    assert!(Settings::parse(&["a"]).a);
    assert!(!Settings::parse(&[]).a);
}

#[test]
fn two_flags() {
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    struct Settings {
        #[flag]
        a: bool,
        #[flag]
        b: bool,
    }

    assert_eq!(Settings::parse(&["a"]), Settings { a: true, b: false });
    assert_eq!(Settings::parse(&[]), Settings { a: false, b: false });
    assert_eq!(Settings::parse(&["b"]), Settings { a: true, b: true });
    assert_eq!(Settings::parse(&["a", "b"]), Settings { a: true, b: true });
}
