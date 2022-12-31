use uutils_args::{Arguments, Options};

#[test]
fn one_positional() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[positional(1)]
        File1(String),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[set(Arg::File1)]
        file1: String,
    }

    let settings = Settings::parse(["test", "foo"]);
    assert_eq!(settings.file1, "foo");

    assert!(Settings::try_parse(["test"]).is_err());
}

#[test]
fn two_positionals() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[positional(1)]
        Foo(String),
        #[positional(1)]
        Bar(String),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[set(Arg::Foo)]
        foo: String,
        #[set(Arg::Bar)]
        bar: String,
    }

    let settings = Settings::parse(["test", "a", "b"]);
    assert_eq!(settings.foo, "a");
    assert_eq!(settings.bar, "b");

    assert!(Settings::try_parse(["test"]).is_err());
}

#[test]
fn optional_positional() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[positional(0..=1)]
        Foo(String),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Foo(s) => Some(s))]
        foo: Option<String>,
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

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[collect(set(Arg::Foo))]
        foo: Vec<String>,
    }

    let settings = Settings::parse(["test", "a", "b", "c"]);
    assert_eq!(settings.foo, vec!["a", "b", "c"]);
    let settings = Settings::parse(["test"]);
    assert_eq!(settings.foo, Vec::<String>::new());
}

#[test]
fn last1() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[positional(last, ..)]
        Foo(Vec<String>),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[set(Arg::Foo)]
        foo: Vec<String>,
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

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[set(Arg::Foo)]
        foo: Vec<String>,
    }

    let settings = Settings::parse(["test", "-a"]);
    assert_eq!(settings.foo, Vec::<String>::new());

    let settings = Settings::parse(["test", "--", "-a"]);
    assert_eq!(settings.foo, vec!["-a"]);
}
