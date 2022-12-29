use uutils_args::{Arguments, Options};

#[test]
fn true_default() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("--foo")]
        Foo,
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[map(Arg::Foo => false)]
        #[field(default = true)]
        foo: bool,
    }

    assert!(Settings::parse(["test"]).unwrap().foo);
    assert!(!Settings::parse(["test", "--foo"]).unwrap().foo);
}

#[test]
fn env_var_string() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("--foo=MSG")]
        Foo(String),
    }

    #[derive(Default, Options)]
    #[arg_type(Arg)]
    struct Settings {
        #[set(Arg::Foo)]
        #[field(env = "FOO")]
        foo: String,
    }

    std::env::set_var("FOO", "one");
    assert_eq!(Settings::parse(["test"]).unwrap().foo, "one");

    std::env::set_var("FOO", "two");
    assert_eq!(Settings::parse(["test"]).unwrap().foo, "two");

    std::env::remove_var("FOO");
    assert_eq!(Settings::parse(["test"]).unwrap().foo, "");
}
