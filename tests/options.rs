use uutils_args::{Arguments, FromValue, Options};

#[test]
fn string_option() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option]
        Message(String),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[set(Arg::Message)]
        message: String,
    }

    assert_eq!(
        Settings::parse(["test", "--message=hello"])
            .unwrap()
            .message,
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

    #[derive(Arguments, Clone)]
    enum Arg {
        #[option]
        Format(Format),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[set(Arg::Format)]
        format: Format,
    }

    assert_eq!(
        Settings::parse(["test", "--format=bar"]).unwrap().format,
        Format::Bar
    );

    assert_eq!(
        Settings::parse(["test", "--format", "baz"]).unwrap().format,
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

    #[derive(Arguments, Clone)]
    enum Arg {
        #[option]
        Indent(Indent),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[set(Arg::Indent)]
        indent: Indent,
    }

    assert_eq!(
        Settings::parse(["test", "-i=thin"]).unwrap().indent,
        Indent::Spaces(4)
    );
    assert_eq!(
        Settings::parse(["test", "-i=wide"]).unwrap().indent,
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

    #[derive(Arguments, Clone)]
    enum Arg {
        #[option]
        Indent(Indent),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Indent(i) => i.clone())]
        indent: Indent,
    }

    assert_eq!(
        Settings::parse(["test", "-i=tabs"]).unwrap().indent,
        Indent::Tabs
    );
    assert_eq!(
        Settings::parse(["test", "-i=4"]).unwrap().indent,
        Indent::Spaces(4)
    );
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

    #[derive(Arguments, Clone)]
    enum Arg {
        #[option]
        Color(Option<Color>),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(
            Arg::Color(Some(c)) => c.clone(),
            Arg::Color(None) => Color::Always,
        )]
        color: Color,
    }

    assert_eq!(
        Settings::parse(["test", "--color=yes"]).unwrap().color,
        Color::Always
    );
    assert_eq!(
        Settings::parse(["test", "--color=always"]).unwrap().color,
        Color::Always
    );
    assert_eq!(
        Settings::parse(["test", "--color=no"]).unwrap().color,
        Color::Never
    );
    assert_eq!(
        Settings::parse(["test", "--color=never"]).unwrap().color,
        Color::Never
    );
    assert_eq!(
        Settings::parse(["test", "--color=auto"]).unwrap().color,
        Color::Auto
    );
    assert_eq!(
        Settings::parse(["test", "--color"]).unwrap().color,
        Color::Always
    )
}

#[test]
fn actions() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option]
        Message(String),
        #[option]
        Send,
        #[option]
        Receive,
    }

    #[derive(Options, Default)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Message(m) => m.clone())]
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

    let settings = Settings::parse(["test", "-m=Hello", "-m=World", "--send"]).unwrap();
    assert_eq!(settings.messages, vec!["Hello", "World"]);
    assert_eq!(settings.message1, "World");
    assert_eq!(settings.message2, "World");
    assert!(settings.send);
}

#[test]
fn width() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option]
        Width(u64),
    }

    #[derive(Options, Default)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(
            Arg::Width(0) => None,
            Arg::Width(x) => Some(x),
        )]
        width: Option<u64>,
    }

    assert_eq!(Settings::parse(["test", "-w=0"]).unwrap().width, None);
    assert_eq!(Settings::parse(["test", "-w=1"]).unwrap().width, Some(1));
}

#[test]
fn integers() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option]
        U8(u8),
        #[option]
        U16(u16),
        #[option]
        U32(u32),
        #[option]
        U64(u64),
        #[option]
        U128(u128),
        #[option]
        I8(i8),
        #[option]
        I16(i16),
        #[option]
        I32(i32),
        #[option]
        I64(i64),
        #[option]
        I128(i128),
    }

    #[derive(Options, Default)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(
            Arg::U8(x) => x as i128,
            Arg::U16(x) => x as i128,
            Arg::U32(x) => x as i128,
            Arg::U64(x) => x as i128,
            Arg::U128(x) => x as i128,
            Arg::I8(x) => x as i128,
            Arg::I16(x) => x as i128,
            Arg::I32(x) => x as i128,
            Arg::I64(x) => x as i128,
            Arg::I128(x) => x as i128,
        )]
        n: i128,
    }

    assert_eq!(Settings::parse(["test", "--u8=5"]).unwrap().n, 5);
    assert_eq!(Settings::parse(["test", "--u16=5"]).unwrap().n, 5);
    assert_eq!(Settings::parse(["test", "--u32=5"]).unwrap().n, 5);
    assert_eq!(Settings::parse(["test", "--u64=5"]).unwrap().n, 5);
    assert_eq!(Settings::parse(["test", "--u128=5"]).unwrap().n, 5);

    assert_eq!(Settings::parse(["test", "--i8=5"]).unwrap().n, 5);
    assert_eq!(Settings::parse(["test", "--i16=5"]).unwrap().n, 5);
    assert_eq!(Settings::parse(["test", "--i32=5"]).unwrap().n, 5);
    assert_eq!(Settings::parse(["test", "--i64=5"]).unwrap().n, 5);
    assert_eq!(Settings::parse(["test", "--i128=5"]).unwrap().n, 5);
}
