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
        #[flag("-b")]
        foo: bool,
    }

    assert_eq!(Settings::parse(["-b"]).unwrap(), Settings { foo: true },);
}

#[test]
fn long_alias() {
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    struct Settings {
        #[flag("--bar")]
        foo: bool,
    }

    assert_eq!(Settings::parse(["--bar"]).unwrap(), Settings { foo: true },);
}

#[test]
fn short_and_long_alias() {
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    struct Settings {
        #[flag("-b", "--bar")]
        foo: bool,
        #[flag("-f", "--foo")]
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
        #[flag("-x", "-z")]
        a: bool,
        #[flag("-x", "-y", "-z")]
        b: bool,
        #[flag("-y", "-z")]
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

#[test]
fn non_rust_ident() {
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    struct Settings {
        #[flag("--foo-bar")]
        a: bool,
        #[flag("--super")]
        b: bool,
    }

    assert_eq!(
        Settings::parse(["--foo-bar", "--super"]).unwrap(),
        Settings { a: true, b: true }
    )
}

#[test]
fn number_flag() {
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    struct Settings {
        #[flag("-1")]
        one: bool,
    }

    assert_eq!(Settings::parse(["-1"]).unwrap(), Settings { one: true })
}

#[test]
fn false_bool() {
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    struct Settings {
        #[flag("-a")]
        #[flag("-b", value = false)]
        foo: bool,
    }

    assert_eq!(Settings::parse(["-a"]).unwrap(), Settings { foo: true });
    assert_eq!(Settings::parse(["-b"]).unwrap(), Settings { foo: false });
    assert_eq!(Settings::parse(["-ab"]).unwrap(), Settings { foo: false });
    assert_eq!(Settings::parse(["-ba"]).unwrap(), Settings { foo: true });
    assert_eq!(
        Settings::parse(["-a", "-b"]).unwrap(),
        Settings { foo: false }
    );
    assert_eq!(
        Settings::parse(["-b", "-a"]).unwrap(),
        Settings { foo: true }
    );
}

#[test]
fn enum_flag() {
    #[derive(Default, PartialEq, Eq, Debug)]
    enum SomeEnum {
        #[default]
        VariantFoo,
        VariantBar,
        VariantBaz,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    struct Settings {
        #[flag(value = SomeEnum::VariantFoo)]
        #[flag("--bar", value = SomeEnum::VariantBar)]
        #[flag("--baz", value = SomeEnum::VariantBaz)]
        foo: SomeEnum,
    }

    assert_eq!(
        Settings::parse(&[] as &[&str]).unwrap(),
        Settings {
            foo: SomeEnum::VariantFoo
        }
    );

    assert_eq!(
        Settings::parse(["--bar"]).unwrap(),
        Settings {
            foo: SomeEnum::VariantBar
        }
    );

    assert_eq!(
        Settings::parse(["--baz"]).unwrap(),
        Settings {
            foo: SomeEnum::VariantBaz
        }
    );
}

#[test]
fn count() {
    #[derive(Default, Options)]
    struct Settings {
        #[flag(value = self.verbosity + 1)]
        verbosity: u8,
    }

    assert_eq!(Settings::parse(["-v"]).unwrap().verbosity, 1);
    assert_eq!(Settings::parse(["-vv"]).unwrap().verbosity, 2);
    assert_eq!(Settings::parse(["-vvv"]).unwrap().verbosity, 3);
}
