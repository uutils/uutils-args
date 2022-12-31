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

    let settings = Settings::parse(["test", "-f"]);
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
        Settings::parse(["test", "-a"]),
        Settings { a: true, b: false }
    );
    assert_eq!(Settings::parse(["test"]), Settings { a: false, b: false });
    assert_eq!(
        Settings::parse(["test", "-b"]),
        Settings { a: false, b: true }
    );
    assert_eq!(
        Settings::parse(["test", "-a", "-b"]),
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

    assert_eq!(Settings::parse(["test"]), Settings { foo: false },);
    assert_eq!(Settings::parse(["test", "--foo"]), Settings { foo: true },);
    assert_eq!(Settings::parse(["test", "-f"]), Settings { foo: true },);
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

    assert_eq!(Settings::parse(["test", "-b"]), Settings { foo: true },);
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

    assert_eq!(Settings::parse(["test", "--bar"]), Settings { foo: true },);
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

    assert_eq!(Settings::parse(["test", "--bar"]), foo_true);
    assert_eq!(Settings::parse(["test", "-b"]), foo_true);
    assert_eq!(Settings::parse(["test", "--foo"]), bar_true);
    assert_eq!(Settings::parse(["test", "-f"]), bar_true);
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
        Settings::parse(["test", "-x"]),
        Settings {
            a: true,
            b: true,
            c: false,
        },
    );

    assert_eq!(
        Settings::parse(["test", "-y"]),
        Settings {
            a: false,
            b: true,
            c: true,
        },
    );

    assert_eq!(
        Settings::parse(["test", "-xy"]),
        Settings {
            a: true,
            b: true,
            c: true,
        },
    );

    assert_eq!(
        Settings::parse(["test", "-z"]),
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
        Settings::parse(["test", "--foo-bar", "--super"]),
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

    assert_eq!(Settings::parse(["test", "-1"]), Settings { one: true })
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

    assert_eq!(Settings::parse(["test", "-a"]), Settings { foo: true });
    assert_eq!(Settings::parse(["test", "-b"]), Settings { foo: false });
    assert_eq!(Settings::parse(["test", "-ab"]), Settings { foo: false });
    assert_eq!(Settings::parse(["test", "-ba"]), Settings { foo: true });
    assert_eq!(
        Settings::parse(["test", "-a", "-b"]),
        Settings { foo: false }
    );
    assert_eq!(
        Settings::parse(["test", "-b", "-a"]),
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

    assert_eq!(Settings2::parse(["test", "-a"]), Settings2 { foo: true });
    assert_eq!(Settings2::parse(["test", "-b"]), Settings2 { foo: false });
    assert_eq!(Settings2::parse(["test", "-ab"]), Settings2 { foo: false });
    assert_eq!(Settings2::parse(["test", "-ba"]), Settings2 { foo: true });
    assert_eq!(
        Settings2::parse(["test", "-a", "-b"]),
        Settings2 { foo: false }
    );
    assert_eq!(
        Settings2::parse(["test", "-b", "-a"]),
        Settings2 { foo: true }
    );
}

#[test]
fn enum_flag() {
    #[derive(Default, PartialEq, Eq, Debug, Clone)]
    enum SomeEnum {
        #[default]
        Foo,
        Bar,
        Baz,
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
            Arg::Foo => SomeEnum::Foo,
            Arg::Bar => SomeEnum::Bar,
            Arg::Baz => SomeEnum::Baz,
        )]
        foo: SomeEnum,
    }

    assert_eq!(Settings::parse(&[] as &[&str]).foo, SomeEnum::Foo);

    assert_eq!(Settings::parse(["test", "--bar"]).foo, SomeEnum::Bar);

    assert_eq!(Settings::parse(["test", "--baz"]).foo, SomeEnum::Baz,);
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

    assert_eq!(Settings::parse(["test", "-v"]).verbosity, 1);
    assert_eq!(Settings::parse(["test", "-vv"]).verbosity, 2);
    assert_eq!(Settings::parse(["test", "-vvv"]).verbosity, 3);
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

    assert!(Settings::parse(["test", "--all"]).all);
    assert!(Settings::parse(["test", "--alm"]).almost_all);
    assert!(Settings::parse(["test", "--au"]).author);
    assert!(Settings::try_parse(["test", "--a"]).is_err());
}
