use uutils_args::{Arguments, Options};

#[test]
fn one_flag() {
    #[derive(Arguments, Clone, Debug, PartialEq, Eq)]
    enum Arg {
        #[option("-f", "--foo")]
        Foo,
    }

    #[derive(Options, Default)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Foo => true)]
        foo: bool,
    }

    let settings = Settings::parse(["test", "-f"]).unwrap();
    assert!(settings.foo);
}

#[test]
fn two_flags() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("-a")]
        A,
        #[option("-b")]
        B,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::A => true)]
        a: bool,
        #[map(Arg::B => true)]
        b: bool,
    }

    assert_eq!(
        Settings::parse(["test", "-a"]).unwrap(),
        Settings { a: true, b: false }
    );
    assert_eq!(
        Settings::parse::<&[&std::ffi::OsStr]>(&[]).unwrap(),
        Settings { a: false, b: false }
    );
    assert_eq!(
        Settings::parse(["test", "-b"]).unwrap(),
        Settings { a: false, b: true }
    );
    assert_eq!(
        Settings::parse(["test", "-a", "-b"]).unwrap(),
        Settings { a: true, b: true }
    );
}

#[test]
fn long_and_short_flag() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("-f", "--foo")]
        Foo,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Foo => true)]
        foo: bool,
    }

    assert_eq!(
        Settings::parse::<&[&std::ffi::OsStr]>(&[]).unwrap(),
        Settings { foo: false },
    );
    assert_eq!(
        Settings::parse(["test", "--foo"]).unwrap(),
        Settings { foo: true },
    );
    assert_eq!(
        Settings::parse(["test", "-f"]).unwrap(),
        Settings { foo: true },
    );
}

#[test]
fn short_alias() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("-b")]
        Foo,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Foo => true)]
        foo: bool,
    }

    assert_eq!(
        Settings::parse(["test", "-b"]).unwrap(),
        Settings { foo: true },
    );
}

#[test]
fn long_alias() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("--bar")]
        Foo,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Foo => true)]
        foo: bool,
    }

    assert_eq!(
        Settings::parse(["test", "--bar"]).unwrap(),
        Settings { foo: true },
    );
}

#[test]
fn short_and_long_alias() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("-b", "--bar")]
        Foo,
        #[option("-f", "--foo")]
        Bar,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Foo => true)]
        foo: bool,
        #[map(Arg::Bar => true)]
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

    assert_eq!(Settings::parse(["test", "--bar"]).unwrap(), foo_true);
    assert_eq!(Settings::parse(["test", "-b"]).unwrap(), foo_true);
    assert_eq!(Settings::parse(["test", "--foo"]).unwrap(), bar_true);
    assert_eq!(Settings::parse(["test", "-f"]).unwrap(), bar_true);
}

#[test]
fn xyz_map_to_abc() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("-x")]
        X,
        #[option("-y")]
        Y,
        #[option("-z")]
        Z,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::X | Arg::Z => true)]
        a: bool,
        #[map(Arg::X | Arg::Y | Arg::Z => true)]
        b: bool,
        #[map(Arg::Y | Arg::Z => true)]
        c: bool,
    }

    assert_eq!(
        Settings::parse(["test", "-x"]).unwrap(),
        Settings {
            a: true,
            b: true,
            c: false,
        },
    );

    assert_eq!(
        Settings::parse(["test", "-y"]).unwrap(),
        Settings {
            a: false,
            b: true,
            c: true,
        },
    );

    assert_eq!(
        Settings::parse(["test", "-xy"]).unwrap(),
        Settings {
            a: true,
            b: true,
            c: true,
        },
    );

    assert_eq!(
        Settings::parse(["test", "-z"]).unwrap(),
        Settings {
            a: true,
            b: true,
            c: true,
        },
    );
}

#[test]
fn non_rust_ident() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("--foo-bar")]
        FooBar,
        #[option("--super")]
        Super,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::FooBar => true)]
        a: bool,
        #[map(Arg::Super => true)]
        b: bool,
    }

    assert_eq!(
        Settings::parse(["test", "--foo-bar", "--super"]).unwrap(),
        Settings { a: true, b: true }
    )
}

#[test]
fn number_flag() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("-1")]
        One,
    }
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::One => true)]
        one: bool,
    }

    assert_eq!(
        Settings::parse(["test", "-1"]).unwrap(),
        Settings { one: true }
    )
}

#[test]
fn false_bool() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("-a")]
        A,
        #[option("-b")]
        B,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(
            Arg::A => true,
            Arg::B => false,
        )]
        foo: bool,
    }

    assert_eq!(
        Settings::parse(["test", "-a"]).unwrap(),
        Settings { foo: true }
    );
    assert_eq!(
        Settings::parse(["test", "-b"]).unwrap(),
        Settings { foo: false }
    );
    assert_eq!(
        Settings::parse(["test", "-ab"]).unwrap(),
        Settings { foo: false }
    );
    assert_eq!(
        Settings::parse(["test", "-ba"]).unwrap(),
        Settings { foo: true }
    );
    assert_eq!(
        Settings::parse(["test", "-a", "-b"]).unwrap(),
        Settings { foo: false }
    );
    assert_eq!(
        Settings::parse(["test", "-b", "-a"]).unwrap(),
        Settings { foo: true }
    );

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings2 {
        #[map(
            Arg::A => true,
            Arg::B => false,
        )]
        foo: bool,
    }

    assert_eq!(
        Settings2::parse(["test", "-a"]).unwrap(),
        Settings2 { foo: true }
    );
    assert_eq!(
        Settings2::parse(["test", "-b"]).unwrap(),
        Settings2 { foo: false }
    );
    assert_eq!(
        Settings2::parse(["test", "-ab"]).unwrap(),
        Settings2 { foo: false }
    );
    assert_eq!(
        Settings2::parse(["test", "-ba"]).unwrap(),
        Settings2 { foo: true }
    );
    assert_eq!(
        Settings2::parse(["test", "-a", "-b"]).unwrap(),
        Settings2 { foo: false }
    );
    assert_eq!(
        Settings2::parse(["test", "-b", "-a"]).unwrap(),
        Settings2 { foo: true }
    );
}

#[test]
fn enum_flag() {
    #[derive(Default, PartialEq, Eq, Debug, Clone)]
    enum SomeEnum {
        #[default]
        VariantFoo,
        VariantBar,
        VariantBaz,
    }

    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("--foo")]
        Foo,
        #[option("--bar")]
        Bar,
        #[option("--baz")]
        Baz,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(
            Arg::Foo => SomeEnum::VariantFoo,
            Arg::Bar => SomeEnum::VariantBar,
            Arg::Baz => SomeEnum::VariantBaz,
        )]
        foo: SomeEnum,
    }

    assert_eq!(
        Settings::parse(&[] as &[&str]).unwrap().foo,
        SomeEnum::VariantFoo
    );

    assert_eq!(
        Settings::parse(["test", "--bar"]).unwrap().foo,
        SomeEnum::VariantBar
    );

    assert_eq!(
        Settings::parse(["test", "--baz"]).unwrap().foo,
        SomeEnum::VariantBaz,
    );
}

#[test]
fn count() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("-v")]
        Verbosity,
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Verbosity => self.verbosity + 1)]
        verbosity: u8,
    }

    assert_eq!(Settings::parse(["test", "-v"]).unwrap().verbosity, 1);
    assert_eq!(Settings::parse(["test", "-vv"]).unwrap().verbosity, 2);
    assert_eq!(Settings::parse(["test", "-vvv"]).unwrap().verbosity, 3);
}

#[test]
fn infer_long_args() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("--all")]
        All,
        #[option("--almost-all")]
        AlmostAll,
        #[option("--author")]
        Author,
    }

    #[derive(Options, Default)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::All => true)]
        all: bool,

        #[map(Arg::AlmostAll => true)]
        almost_all: bool,

        #[map(Arg::Author => true)]
        author: bool,
    }

    assert!(Settings::parse(["test", "--all"]).unwrap().all);
    assert!(Settings::parse(["test", "--alm"]).unwrap().almost_all);
    assert!(Settings::parse(["test", "--au"]).unwrap().author);
    assert!(Settings::parse(["test", "--a"]).is_err());
}
