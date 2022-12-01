use uutils_args::{ArgumentIter, Arguments, FromValue, Options};

fn to_vec<T: Arguments>(mut args: ArgumentIter<T>) -> Vec<T> {
    let mut v = Vec::new();
    while let Some(arg) = args.next_arg().unwrap() {
        v.push(arg);
    }
    v
}

#[test]
fn one_flag() {
    #[derive(Arguments, Debug, PartialEq, Eq)]
    enum Arg {
        #[flag]
        Foo,
    }

    #[derive(Options, Default)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Foo => true)]
        foo: bool,
    }

    let iter = Arg::parse(["-f", "--foo"]);
    assert_eq!(to_vec(iter), vec![Arg::Foo, Arg::Foo]);

    let settings = Settings::parse(["-f"]).unwrap();
    assert!(settings.foo);
}

#[test]
fn two_flags() {
    #[derive(Arguments)]
    enum Arg {
        #[flag]
        A,
        #[flag]
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
    #[derive(Arguments)]
    enum Arg {
        #[flag]
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
    assert_eq!(Settings::parse(["--foo"]).unwrap(), Settings { foo: true },);
    assert_eq!(Settings::parse(["-f"]).unwrap(), Settings { foo: true },);
}

#[test]
fn short_alias() {
    #[derive(Arguments)]
    enum Arg {
        #[flag("-b")]
        Foo,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Foo => true)]
        foo: bool,
    }

    assert_eq!(Settings::parse(["-b"]).unwrap(), Settings { foo: true },);
}

#[test]
fn long_alias() {
    #[derive(Arguments)]
    enum Arg {
        #[flag("--bar")]
        Foo,
    }

    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Foo => true)]
        foo: bool,
    }

    assert_eq!(Settings::parse(["--bar"]).unwrap(), Settings { foo: true },);
}

#[test]
fn short_and_long_alias() {
    #[derive(Arguments)]
    enum Arg {
        #[flag("-b", "--bar")]
        Foo,
        #[flag("-f", "--foo")]
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

    assert_eq!(Settings::parse(["--bar"]).unwrap(), foo_true);
    assert_eq!(Settings::parse(["-b"]).unwrap(), foo_true);
    assert_eq!(Settings::parse(["--foo"]).unwrap(), bar_true);
    assert_eq!(Settings::parse(["-f"]).unwrap(), bar_true);
}

#[test]
fn xyz_map_to_abc() {
    #[derive(Arguments)]
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
        #[map(Arg::X | Arg::Z => true)]
        a: bool,
        #[map(Arg::X | Arg::Y | Arg::Z => true)]
        b: bool,
        #[map(Arg::Y | Arg::Z => true)]
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
    #[derive(Arguments)]
    enum Arg {
        #[flag("--foo-bar")]
        FooBar,
        #[flag("--super")]
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
        Settings::parse(["--foo-bar", "--super"]).unwrap(),
        Settings { a: true, b: true }
    )
}

#[test]
fn number_flag() {
    #[derive(Arguments)]
    enum Arg {
        #[flag("-1")]
        One,
    }
    #[derive(Default, Options, PartialEq, Eq, Debug)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::One => true)]
        one: bool,
    }

    assert_eq!(Settings::parse(["-1"]).unwrap(), Settings { one: true })
}

#[test]
fn false_bool() {
    #[derive(Arguments)]
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
            Arg::B => false
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

    #[derive(Arguments)]
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
    #[derive(Arguments)]
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
fn string_option() {
    #[derive(Arguments)]
    enum Arg {
        #[option]
        Message(String),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Message(s) => s)]
        message: String,
    }

    assert_eq!(
        Settings::parse(["--message=hello"]).unwrap().message,
        "hello"
    );
}

#[test]
fn enum_option() {
    #[derive(FromValue, Default, Debug, PartialEq, Eq, Clone)]
    enum Format {
        #[default]
        #[value]
        Foo,
        #[value]
        Bar,
        #[value]
        Baz,
    }

    #[derive(Arguments)]
    enum Arg {
        #[option]
        Format(Format),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Format(f) => f)]
        format: Format,
    }

    assert_eq!(
        Settings::parse(["--format=bar"]).unwrap().format,
        Format::Bar
    );

    assert_eq!(
        Settings::parse(["--format", "baz"]).unwrap().format,
        Format::Baz
    );
}

#[test]
fn enum_option_with_fields() {
    #[derive(FromValue, Default, Debug, PartialEq, Eq, Clone)]
    enum Indent {
        #[default]
        Tabs,
        #[value("thin", value = Self::Spaces(4))]
        #[value("wide", value = Self::Spaces(8))]
        Spaces(u8),
    }

    #[derive(Arguments)]
    enum Arg {
        #[option]
        Indent(Indent),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Indent(i) => i)]
        indent: Indent,
    }

    assert_eq!(
        Settings::parse(["-i=thin"]).unwrap().indent,
        Indent::Spaces(4)
    );
    assert_eq!(
        Settings::parse(["-i=wide"]).unwrap().indent,
        Indent::Spaces(8)
    );
}

#[test]
fn enum_with_complex_from_value() {
    #[derive(Default, Debug, PartialEq, Eq, Clone)]
    enum Indent {
        #[default]
        Tabs,
        Spaces(u8),
    }

    impl FromValue for Indent {
        fn from_value(value: std::ffi::OsString) -> Result<Self, lexopt::Error> {
            let value = value.into_string()?;
            if value == "tabs" {
                Ok(Self::Tabs)
            } else if let Ok(n) = value.parse() {
                Ok(Self::Spaces(n))
            } else {
                Err(lexopt::Error::ParsingFailed {
                    value,
                    error: "Failure!".into(),
                })
            }
        }
    }

    #[derive(Arguments)]
    enum Arg {
        #[option]
        Indent(Indent),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Indent(i) => i)]
        indent: Indent,
    }

    assert_eq!(Settings::parse(["-i=tabs"]).unwrap().indent, Indent::Tabs);
    assert_eq!(Settings::parse(["-i=4"]).unwrap().indent, Indent::Spaces(4));
}

#[test]
fn color() {
    #[derive(Default, FromValue, Debug, PartialEq, Eq, Clone)]
    enum Color {
        #[value("yes", "always")]
        Always,
        #[default]
        #[value("auto")]
        Auto,
        #[value("no", "never")]
        Never,
    }

    #[derive(Arguments)]
    enum Arg {
        #[option]
        Color(Color),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Color(c) => c)]
        color: Color,
    }

    assert_eq!(
        Settings::parse(["--color=yes"]).unwrap().color,
        Color::Always
    );
    assert_eq!(
        Settings::parse(["--color=always"]).unwrap().color,
        Color::Always
    );
    assert_eq!(Settings::parse(["--color=no"]).unwrap().color, Color::Never);
    assert_eq!(
        Settings::parse(["--color=never"]).unwrap().color,
        Color::Never
    );
    assert_eq!(
        Settings::parse(["--color=auto"]).unwrap().color,
        Color::Auto
    );
}

#[test]
fn actions() {
    #[derive(Arguments)]
    enum Arg {
        #[option]
        Message(String),
        #[flag]
        Send,
        #[flag]
        Receive,
    }

    #[derive(Options, Default)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Message(m) => m)]
        message1: String,

        #[set(Arg::Message)]
        message2: String,

        #[set_true(Arg::Send)]
        #[set_false(Arg::Receive)]
        send: bool,

        // Or map, true or false inside the collect
        #[collect(set(Arg::Message))]
        messages: Vec<String>,
    }

    let settings = Settings::parse(["-m=Hello", "-m=World", "--send"]).unwrap();
    assert_eq!(settings.messages, vec!["Hello", "World"]);
    assert_eq!(settings.message1, "World");
    assert_eq!(settings.message2, "World");
    assert!(settings.send);
}
