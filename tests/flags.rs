use uutils_args::{Argument, ArgumentIter, Arguments, Options};

fn to_vec<T: Arguments>(mut args: ArgumentIter<T>) -> Vec<T> {
    let mut v = Vec::new();
    while let Some(Argument::Custom(arg)) = args.next_arg().unwrap() {
        v.push(arg);
    }
    v
}

#[test]
fn one_flag() {
    #[derive(Arguments, Clone, Debug, PartialEq, Eq)]
    enum Arg {
        #[option]
        Foo,
    }

    #[derive(Options, Default)]
    #[arg_type(Arg)]
    struct Settings {
        #[set_true(Arg::Foo)]
        foo: bool,
    }

    let iter = Arg::parse(["test", "-f", "--foo"]);
    assert_eq!(to_vec(iter), vec![Arg::Foo, Arg::Foo]);

    let settings = Settings::parse(["test", "-f"]).unwrap();
    assert!(settings.foo);
}

#[test]
fn two_flags() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option]
        A,
        #[option]
        B,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[set_true(Arg::A)]
        a: bool,
        #[set_true(Arg::B)]
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
        #[option]
        Foo,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[set_true(Arg::Foo)]
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
        #[set_true(Arg::Foo)]
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
        #[set_true(Arg::Foo)]
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
        #[set_true(Arg::Foo)]
        foo: bool,
        #[set_true(Arg::Bar)]
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
        #[option]
        X,
        #[option]
        Y,
        #[option]
        Z,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[set_true(Arg::X | Arg::Z)]
        a: bool,
        #[set_true(Arg::X | Arg::Y | Arg::Z)]
        b: bool,
        #[set_true(Arg::Y | Arg::Z)]
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
        #[set_true(Arg::FooBar)]
        a: bool,
        #[set_true(Arg::Super)]
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
        #[set_true(Arg::One)]
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
        #[option]
        A,
        #[option]
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
        #[set_true(Arg::A)]
        #[set_false(Arg::B)]
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
        #[option]
        Foo,
        #[option]
        Bar,
        #[option]
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
        #[option]
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
        #[set_true(Arg::All)]
        all: bool,
        #[set_true(Arg::AlmostAll)]
        almost_all: bool,
        #[set_true(Arg::Author)]
        author: bool,
    }

    assert!(Settings::parse(["test", "--all"]).unwrap().all);
    assert!(Settings::parse(["test", "--alm"]).unwrap().almost_all);
    assert!(Settings::parse(["test", "--au"]).unwrap().author);
    assert!(Settings::parse(["test", "--a"]).is_err());
}
