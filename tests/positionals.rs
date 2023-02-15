use uutils_args::{Arguments, Initial, Options};

#[test]
fn one_positional() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[positional(1)]
        File1(String),
    }

    #[derive(Initial)]
    struct Settings {
        file1: String,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::File1(f): Arg) {
            self.file1 = f;
        }
    }

    let settings = Settings::parse(["test", "foo"]);
    assert_eq!(settings.file1, "foo");

    assert!(Settings::try_parse(["test"]).is_err());
}

#[test]
fn two_positionals() {
    #[derive(Arguments)]
    enum Arg {
        #[positional(1)]
        Foo(String),
        #[positional(1)]
        Bar(String),
    }

    #[derive(Initial)]
    struct Settings {
        foo: String,
        bar: String,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) {
            match arg {
                Arg::Foo(x) => self.foo = x,
                Arg::Bar(x) => self.bar = x,
            }
        }
    }

    let settings = Settings::parse(["test", "a", "b"]);
    assert_eq!(settings.foo, "a");
    assert_eq!(settings.bar, "b");

    assert!(Settings::try_parse(["test"]).is_err());
}

#[test]
fn optional_positional() {
    #[derive(Arguments)]
    enum Arg {
        #[positional(0..=1)]
        Foo(String),
    }

    #[derive(Initial)]
    struct Settings {
        foo: Option<String>,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Foo(x): Arg) {
            self.foo = Some(x);
        }
    }

    let settings = Settings::parse(["test"]);
    assert_eq!(settings.foo, None);
    let settings = Settings::parse(["test", "bar"]);
    assert_eq!(settings.foo.unwrap(), "bar");
}

#[test]
fn collect_positional() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[positional(..)]
        Foo(String),
    }

    #[derive(Initial)]
    struct Settings {
        foo: Vec<String>,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Foo(x): Arg) {
            self.foo.push(x);
        }
    }

    let settings = Settings::parse(["test", "a", "b", "c"]);
    assert_eq!(settings.foo, vec!["a", "b", "c"]);
    let settings = Settings::parse(["test"]);
    assert_eq!(settings.foo, Vec::<String>::new());
}

#[test]
fn last1() {
    #[derive(Arguments)]
    enum Arg {
        #[positional(last, ..)]
        Foo(Vec<String>),
    }

    #[derive(Initial)]
    struct Settings {
        foo: Vec<String>,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Foo(x): Arg) {
            self.foo = x;
        }
    }

    let settings = Settings::parse(["test", "a", "-b", "c"]);
    assert_eq!(settings.foo, vec!["a", "-b", "c"]);
}

#[test]
fn last2() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("-a")]
        A,

        #[positional(last, ..)]
        Foo(Vec<String>),
    }

    #[derive(Initial)]
    struct Settings {
        foo: Vec<String>,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) {
            match arg {
                Arg::A => {}
                Arg::Foo(x) => self.foo = x,
            }
        }
    }

    let settings = Settings::parse(["test", "-a"]);
    assert_eq!(settings.foo, Vec::<String>::new());

    let settings = Settings::parse(["test", "--", "-a"]);
    assert_eq!(settings.foo, vec!["-a"]);
}
