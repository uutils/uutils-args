use std::ffi::OsStr;

use uutils_args::{Arguments, Options, Value, ValueResult};

#[test]
fn string_option() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("--message=MSG")]
        Message(String),
    }

    #[derive(Default)]
    struct Settings {
        message: String,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Message(s): Arg) -> Result<(), uutils_args::Error> {
            self.message = s;
            Ok(())
        }
    }

    assert_eq!(
        Settings::default()
            .parse(["test", "--message=hello"])
            .unwrap()
            .0
            .message,
        "hello"
    );
}

#[test]
fn enum_option() {
    #[derive(Value, Default, Debug, PartialEq, Eq, Clone)]
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
        #[arg("--format=FORMAT")]
        Format(Format),
    }

    #[derive(Default)]
    struct Settings {
        format: Format,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Format(f): Arg) -> Result<(), uutils_args::Error> {
            self.format = f;
            Ok(())
        }
    }

    assert_eq!(
        Settings::default()
            .parse(["test", "--format=bar"])
            .unwrap()
            .0
            .format,
        Format::Bar
    );

    assert_eq!(
        Settings::default()
            .parse(["test", "--format", "baz"])
            .unwrap()
            .0
            .format,
        Format::Baz
    );
}

#[test]
fn enum_option_with_fields() {
    #[derive(Value, Default, Debug, PartialEq, Eq)]
    enum Indent {
        #[default]
        Tabs,
        #[value("thin", value = Self::Spaces(4))]
        #[value("wide", value = Self::Spaces(8))]
        Spaces(u8),
    }

    #[derive(Arguments)]
    enum Arg {
        #[arg("-i INDENT")]
        Indent(Indent),
    }

    #[derive(Default)]
    struct Settings {
        indent: Indent,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Indent(i): Arg) -> Result<(), uutils_args::Error> {
            self.indent = i;
            Ok(())
        }
    }

    assert_eq!(
        Settings::default()
            .parse(["test", "-i=thin"])
            .unwrap()
            .0
            .indent,
        Indent::Spaces(4)
    );
    assert_eq!(
        Settings::default()
            .parse(["test", "-i=wide"])
            .unwrap()
            .0
            .indent,
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

    impl Value for Indent {
        fn from_value(value: &std::ffi::OsStr) -> ValueResult<Self> {
            let value = String::from_value(value)?;
            if value == "tabs" {
                Ok(Self::Tabs)
            } else if let Ok(n) = value.parse() {
                Ok(Self::Spaces(n))
            } else {
                Err("Failure!".into())
            }
        }
    }

    #[derive(Arguments)]
    enum Arg {
        #[arg("-i INDENT")]
        Indent(Indent),
    }

    #[derive(Default)]
    struct Settings {
        indent: Indent,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Indent(i): Arg) -> Result<(), uutils_args::Error> {
            self.indent = i;
            Ok(())
        }
    }

    assert_eq!(
        Settings::default()
            .parse(["test", "-i=tabs"])
            .unwrap()
            .0
            .indent,
        Indent::Tabs
    );
    assert_eq!(
        Settings::default()
            .parse(["test", "-i=4"])
            .unwrap()
            .0
            .indent,
        Indent::Spaces(4)
    );
}

#[test]
fn color() {
    #[derive(Default, Value, Debug, PartialEq, Eq)]
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
        #[arg("--color[=WHEN]")]
        Color(Option<Color>),
    }

    #[derive(Default)]
    struct Settings {
        color: Color,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Color(c): Arg) -> Result<(), uutils_args::Error> {
            self.color = c.unwrap_or(Color::Always);
            Ok(())
        }
    }

    assert_eq!(
        Settings::default()
            .parse(["test", "--color=yes"])
            .unwrap()
            .0
            .color,
        Color::Always
    );
    assert_eq!(
        Settings::default()
            .parse(["test", "--color=always"])
            .unwrap()
            .0
            .color,
        Color::Always
    );
    assert_eq!(
        Settings::default()
            .parse(["test", "--color=no"])
            .unwrap()
            .0
            .color,
        Color::Never
    );
    assert_eq!(
        Settings::default()
            .parse(["test", "--color=never"])
            .unwrap()
            .0
            .color,
        Color::Never
    );
    assert_eq!(
        Settings::default()
            .parse(["test", "--color=auto"])
            .unwrap()
            .0
            .color,
        Color::Auto
    );
    assert_eq!(
        Settings::default()
            .parse(["test", "--color"])
            .unwrap()
            .0
            .color,
        Color::Always
    )
}

#[test]
fn actions() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("-m MESSAGE")]
        Message(String),
        #[arg("--send")]
        Send,
        #[arg("--receive")]
        Receive,
    }

    #[derive(Default)]
    struct Settings {
        last_message: String,
        send: bool,
        messages: Vec<String>,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            match arg {
                Arg::Message(m) => {
                    self.last_message.clone_from(&m);
                    self.messages.push(m);
                }
                Arg::Send => self.send = true,
                Arg::Receive => self.send = false,
            }
            Ok(())
        }
    }

    let (settings, _operands) = Settings::default()
        .parse(["test", "-m=Hello", "-m=World", "--send"])
        .unwrap();
    assert_eq!(settings.messages, vec!["Hello", "World"]);
    assert_eq!(settings.last_message, "World");
    assert!(settings.send);
}

#[test]
fn width() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("-w WIDTH")]
        Width(u64),
    }

    #[derive(Default)]
    struct Settings {
        width: Option<u64>,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Width(w): Arg) -> Result<(), uutils_args::Error> {
            self.width = match w {
                0 => None,
                x => Some(x),
            };
            Ok(())
        }
    }

    assert_eq!(
        Settings::default().parse(["test", "-w=0"]).unwrap().0.width,
        None
    );
    assert_eq!(
        Settings::default().parse(["test", "-w=1"]).unwrap().0.width,
        Some(1)
    );
}

#[test]
fn integers() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("--u8=N")]
        U8(u8),
        #[arg("--u16=N")]
        U16(u16),
        #[arg("--u32=N")]
        U32(u32),
        #[arg("--u64=N")]
        U64(u64),
        #[arg("--u128=N")]
        U128(u128),
        #[arg("--i8=N")]
        I8(i8),
        #[arg("--i16=N")]
        I16(i16),
        #[arg("--i32=N")]
        I32(i32),
        #[arg("--i64=N")]
        I64(i64),
        #[arg("--i128=N")]
        I128(i128),
    }

    #[derive(Default)]
    struct Settings {
        n: i128,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            self.n = match arg {
                Arg::U8(x) => x as i128,
                Arg::U16(x) => x as i128,
                Arg::U32(x) => x as i128,
                Arg::U64(x) => x as i128,
                Arg::U128(x) => x as i128,
                Arg::I8(x) => x as i128,
                Arg::I16(x) => x as i128,
                Arg::I32(x) => x as i128,
                Arg::I64(x) => x as i128,
                Arg::I128(x) => x,
            };
            Ok(())
        }
    }

    assert_eq!(
        Settings::default().parse(["test", "--u8=5"]).unwrap().0.n,
        5
    );
    assert_eq!(
        Settings::default().parse(["test", "--u16=5"]).unwrap().0.n,
        5
    );
    assert_eq!(
        Settings::default().parse(["test", "--u32=5"]).unwrap().0.n,
        5
    );
    assert_eq!(
        Settings::default().parse(["test", "--u64=5"]).unwrap().0.n,
        5
    );
    assert_eq!(
        Settings::default().parse(["test", "--u128=5"]).unwrap().0.n,
        5
    );

    assert_eq!(
        Settings::default().parse(["test", "--i8=5"]).unwrap().0.n,
        5
    );
    assert_eq!(
        Settings::default().parse(["test", "--i16=5"]).unwrap().0.n,
        5
    );
    assert_eq!(
        Settings::default().parse(["test", "--i32=5"]).unwrap().0.n,
        5
    );
    assert_eq!(
        Settings::default().parse(["test", "--i64=5"]).unwrap().0.n,
        5
    );
    assert_eq!(
        Settings::default().parse(["test", "--i128=5"]).unwrap().0.n,
        5
    );
}

#[test]
fn ls_classify() {
    #[derive(Value, Default, PartialEq, Eq, Debug)]
    enum When {
        #[value]
        Never,
        #[default]
        #[value]
        Auto,
        #[value]
        Always,
    }

    #[derive(Arguments)]
    enum Arg {
        #[arg(
            "-F", "--classify[=WHEN]",
            value = When::Always,
        )]
        Classify(When),
    }

    #[derive(Default)]
    struct Settings {
        classify: When,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Classify(c): Arg) -> Result<(), uutils_args::Error> {
            self.classify = c;
            Ok(())
        }
    }

    assert_eq!(
        Settings::default().parse(["test"]).unwrap().0.classify,
        When::Auto
    );
    assert_eq!(
        Settings::default()
            .parse(["test", "--classify=never"])
            .unwrap()
            .0
            .classify,
        When::Never,
    );
    assert_eq!(
        Settings::default()
            .parse(["test", "--classify"])
            .unwrap()
            .0
            .classify,
        When::Always,
    );
    assert_eq!(
        Settings::default()
            .parse(["test", "-F"])
            .unwrap()
            .0
            .classify,
        When::Always,
    );
    assert!(Settings::default().parse(["test", "-Falways"]).is_err());
}

#[test]
fn mktemp_tmpdir() {
    #[derive(Clone, Arguments)]
    enum Arg {
        #[arg(
            "-p DIR", "--tmpdir[=DIR]",
            value = String::from("/tmp"),
        )]
        TmpDir(String),
    }

    #[derive(Default)]
    struct Settings {
        tmpdir: Option<String>,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::TmpDir(dir): Arg) -> Result<(), uutils_args::Error> {
            self.tmpdir = Some(dir);
            Ok(())
        }
    }

    let (settings, _operands) = Settings::default().parse(["test", "-p", "X"]).unwrap();
    assert_eq!(settings.tmpdir.unwrap(), "X");

    let (settings, _operands) = Settings::default().parse(["test", "--tmpdir=X"]).unwrap();
    assert_eq!(settings.tmpdir.unwrap(), "X");

    let (settings, _operands) = Settings::default().parse(["test", "--tmpdir"]).unwrap();
    assert_eq!(settings.tmpdir.unwrap(), "/tmp");

    assert!(Settings::default().parse(["test", "-p"]).is_err());
}

#[test]
fn infer_value() {
    #[derive(Value, PartialEq, Eq, Debug)]
    enum Foo {
        #[value("long")]
        Long,
        #[value("link")]
        Link,
        #[value("deck")]
        Deck,
        #[value("desk")]
        Desk,
    }

    assert_eq!(Foo::from_value(OsStr::new("lo")).unwrap(), Foo::Long);
    assert_eq!(Foo::from_value(OsStr::new("dec")).unwrap(), Foo::Deck);

    Foo::from_value(OsStr::new("l")).unwrap_err();
    Foo::from_value(OsStr::new("de")).unwrap_err();
}

#[test]
fn deprecated() {
    fn parse_minus(s: &str) -> Option<&str> {
        let num = s.strip_prefix('-')?;
        if num.chars().all(|c| c.is_ascii_digit()) {
            Some(num)
        } else {
            None
        }
    }
    fn parse_plus(s: &str) -> Option<&str> {
        let num = s.strip_prefix('+')?;
        let num2 = num.strip_prefix('-').unwrap_or(num);
        if num2.chars().all(|c| c.is_ascii_digit()) {
            Some(num)
        } else {
            None
        }
    }

    #[derive(Arguments)]
    enum Arg {
        #[arg(parse_minus)]
        Min(usize),

        #[arg(parse_plus)]
        Plus(isize),
    }

    #[derive(Default)]
    struct Settings {
        n1: usize,
        n2: isize,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            match arg {
                Arg::Min(n) => self.n1 = n,
                Arg::Plus(n) => self.n2 = n,
            }
            Ok(())
        }
    }

    assert_eq!(
        Settings::default().parse(["test", "-10"]).unwrap().0.n1,
        10usize
    );
    assert!(Settings::default().parse(["test", "--10"]).is_err());
    assert_eq!(
        Settings::default().parse(["test", "+10"]).unwrap().0.n2,
        10isize
    );
    assert_eq!(
        Settings::default().parse(["test", "+-10"]).unwrap().0.n2,
        -10isize
    );
}

#[test]
#[allow(unreachable_code)]
fn empty_value() {
    // We just check that this compiles
    #[derive(Value)]
    enum V {}

    #[derive(Arguments)]
    enum Arg {
        #[arg("--val=VAL")]
        Val(V),
    }

    #[allow(dead_code)]
    struct Settings {}

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            match arg {
                Arg::Val(_) => {}
            }
            Ok(())
        }
    }
}
