use uutils_args::{ArgumentIter, Arguments, Options};

fn to_vec<T: Arguments>(mut args: ArgumentIter<T>) -> Vec<T> {
    let mut v = Vec::new();
    while let Some(arg) = args.next_arg().unwrap() {
        v.push(arg);
    }
    v
}

#[test]
fn one_flag() {
    #[derive(Arguments, Clone, Debug, PartialEq, Eq)]
    enum Arg {
        #[flag]
        Foo,
    }

    #[derive(Options, Default)]
    #[arg_type(Arg)]
    struct Settings {
        #[set_true(Arg::Foo)]
        foo: bool,
    }

    let iter = Arg::parse(["-f", "--foo"]);
    assert_eq!(to_vec(iter), vec![Arg::Foo, Arg::Foo]);

    let settings = Settings::parse(["-f"]).unwrap();
    assert!(settings.foo);
}

#[test]
fn two_flags() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[flag]
        A,
        #[flag]
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
    #[derive(Arguments, Clone)]
    enum Arg {
        #[flag]
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
    assert_eq!(Settings::parse(["--foo"]).unwrap(), Settings { foo: true },);
    assert_eq!(Settings::parse(["-f"]).unwrap(), Settings { foo: true },);
}

#[test]
fn short_alias() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[flag("-b")]
        Foo,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[set_true(Arg::Foo)]
        foo: bool,
    }

    assert_eq!(Settings::parse(["-b"]).unwrap(), Settings { foo: true },);
}

#[test]
fn long_alias() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[flag("--bar")]
        Foo,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[set_true(Arg::Foo)]
        foo: bool,
    }

    assert_eq!(Settings::parse(["--bar"]).unwrap(), Settings { foo: true },);
}

#[test]
fn short_and_long_alias() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[flag("-b", "--bar")]
        Foo,
        #[flag("-f", "--foo")]
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

    assert_eq!(Settings::parse(["--bar"]).unwrap(), foo_true);
    assert_eq!(Settings::parse(["-b"]).unwrap(), foo_true);
    assert_eq!(Settings::parse(["--foo"]).unwrap(), bar_true);
    assert_eq!(Settings::parse(["-f"]).unwrap(), bar_true);
}

#[test]
fn xyz_map_to_abc() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[flag]
        X,
        #[flag]
        Y,
        #[flag]
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
    #[derive(Arguments, Clone)]
    enum Arg {
        #[flag("--foo-bar")]
        FooBar,
        #[flag("--super")]
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
        Settings::parse(["--foo-bar", "--super"]).unwrap(),
        Settings { a: true, b: true }
    )
}

#[test]
fn number_flag() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[flag("-1")]
        One,
    }
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[set_true(Arg::One)]
        one: bool,
    }

    assert_eq!(Settings::parse(["-1"]).unwrap(), Settings { one: true })
}

#[test]
fn false_bool() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[flag]
        A,
        #[flag]
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

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings2 {
        #[set_true(Arg::A)]
        #[set_false(Arg::B)]
        foo: bool,
    }

    assert_eq!(Settings2::parse(["-a"]).unwrap(), Settings2 { foo: true });
    assert_eq!(Settings2::parse(["-b"]).unwrap(), Settings2 { foo: false });
    assert_eq!(Settings2::parse(["-ab"]).unwrap(), Settings2 { foo: false });
    assert_eq!(Settings2::parse(["-ba"]).unwrap(), Settings2 { foo: true });
    assert_eq!(
        Settings2::parse(["-a", "-b"]).unwrap(),
        Settings2 { foo: false }
    );
    assert_eq!(
        Settings2::parse(["-b", "-a"]).unwrap(),
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
        #[flag]
        Foo,
        #[flag]
        Bar,
        #[flag]
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
        Settings::parse(["--bar"]).unwrap().foo,
        SomeEnum::VariantBar
    );

    assert_eq!(
        Settings::parse(["--baz"]).unwrap().foo,
        SomeEnum::VariantBaz,
    );
}

#[test]
fn count() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[flag]
        Verbosity,
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Verbosity => self.verbosity + 1)]
        verbosity: u8,
    }

    assert_eq!(Settings::parse(["-v"]).unwrap().verbosity, 1);
    assert_eq!(Settings::parse(["-vv"]).unwrap().verbosity, 2);
    assert_eq!(Settings::parse(["-vvv"]).unwrap().verbosity, 3);
}

#[test]
fn infer_long_args() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[flag("--all")]
        All,
        #[flag("--almost-all")]
        AlmostAll,
        #[flag("--author")]
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

    assert!(Settings::parse(["--all"]).unwrap().all);
    assert!(Settings::parse(["--alm"]).unwrap().almost_all);
    assert!(Settings::parse(["--au"]).unwrap().author);
    assert!(Settings::parse(["--a"]).is_err());
}
