use uutils_args::Options;

#[test]
fn one_flag() {
    #[derive(Default, Options)]
    struct Settings {
        #[flag]
        a: bool,
    }

    assert!(Settings::parse(["-a"]).unwrap().a);
    assert!(!Settings::parse::<&[&str]>(&[]).unwrap().a);
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

    assert_eq!(
        Settings::parse(["-a"]).unwrap(),
        Settings { a: true, b: false }
    );
    assert_eq!(
        Settings::parse::<&[&std::ffi::OsStr]>(&[]).unwrap(),
        Settings { a: false, b: false }
    );
    assert_eq!(
        Settings::parse(["-b"]).unwrap(),
        Settings { a: false, b: true }
    );
    assert_eq!(
        Settings::parse(["-a", "-b"]).unwrap(),
        Settings { a: true, b: true }
    );
}

#[test]
fn long_and_short_flag() {
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    struct Settings {
        #[flag]
        foo: bool,
    }

    assert_eq!(
        Settings::parse::<&[&std::ffi::OsStr]>(&[]).unwrap(),
        Settings { foo: false },
    );
    assert_eq!(Settings::parse(["--foo"]).unwrap(), Settings { foo: true },);
    assert_eq!(Settings::parse(["-f"]).unwrap(), Settings { foo: true },);
}

#[test]
fn short_alias() {
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    struct Settings {
        #[flag(-b)]
        foo: bool,
    }

    assert_eq!(Settings::parse(["-b"]).unwrap(), Settings { foo: true },);
}

#[test]
fn long_alias() {
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    struct Settings {
        #[flag(--bar)]
        foo: bool,
    }

    assert_eq!(Settings::parse(["--bar"]).unwrap(), Settings { foo: true },);
}

#[test]
fn short_and_long_alias() {
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    struct Settings {
        #[flag(-b, --bar)]
        foo: bool,
        #[flag(-f, --foo)]
        bar: bool,
    }

    let foo_true = Settings {
        foo: true,
        bar: false,
    };

    let bar_true = Settings {
        foo: false,
        bar: true,
    };

    assert_eq!(Settings::parse(["--bar"]).unwrap(), foo_true);
    assert_eq!(Settings::parse(["-b"]).unwrap(), foo_true);
    assert_eq!(Settings::parse(["--foo"]).unwrap(), bar_true);
    assert_eq!(Settings::parse(["-f"]).unwrap(), bar_true);
}

#[test]
fn xyz_map_to_abc() {
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    struct Settings {
        #[flag(-x, -z)]
        a: bool,
        #[flag(-x, -y, -z)]
        b: bool,
        #[flag(-y, -z)]
        c: bool,
    }

    assert_eq!(
        Settings::parse(["-x"]).unwrap(),
        Settings {
            a: true,
            b: true,
            c: false,
        },
    );

    assert_eq!(
        Settings::parse(["-y"]).unwrap(),
        Settings {
            a: false,
            b: true,
            c: true,
        },
    );

    assert_eq!(
        Settings::parse(["-xy"]).unwrap(),
        Settings {
            a: true,
            b: true,
            c: true,
        },
    );

    assert_eq!(
        Settings::parse(["-z"]).unwrap(),
        Settings {
            a: true,
            b: true,
            c: true,
        },
    );
}
