use uutils_args::{Arguments, FromValue, Options};

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
        #[set(Arg::Message)]
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
        #[set(Arg::Format)]
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
        #[set(Arg::Indent)]
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
        Color(Option<Color>),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(
            Arg::Color(Some(c)) => c,
            Arg::Color(None) => Color::Always,
        )]
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
    assert_eq!(Settings::parse(["--color"]).unwrap().color, Color::Always)
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
