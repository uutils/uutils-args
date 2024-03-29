use uutils_args::{Arguments, Options};

#[test]
fn one_flag() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("-f", "--foo")]
        Foo,
    }

    #[derive(Default)]
    struct Settings {
        foo: bool,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            match arg {
                Arg::Foo => self.foo = true,
            }
            Ok(())
        }
    }

    let (settings, _) = Settings::default().parse(["test", "-f"]).unwrap();
    assert!(settings.foo);
}

#[test]
fn two_flags() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[arg("-a")]
        A,
        #[arg("-b")]
        B,
    }

    #[derive(Default, PartialEq, Eq, Debug)]
    struct Settings {
        a: bool,
        b: bool,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            match arg {
                Arg::A => self.a = true,
                Arg::B => self.b = true,
            }
            Ok(())
        }
    }

    assert_eq!(
        Settings::default().parse(["test", "-a"]).unwrap().0,
        Settings { a: true, b: false }
    );
    assert_eq!(
        Settings::default().parse(["test"]).unwrap().0,
        Settings { a: false, b: false }
    );
    assert_eq!(
        Settings::default().parse(["test", "-b"]).unwrap().0,
        Settings { a: false, b: true }
    );
    assert_eq!(
        Settings::default().parse(["test", "-a", "-b"]).unwrap().0,
        Settings { a: true, b: true }
    );
}

#[test]
fn long_and_short_flag() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("-f", "--foo")]
        Foo,
    }

    #[derive(Default)]
    struct Settings {
        foo: bool,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Foo: Arg) -> Result<(), uutils_args::Error> {
            self.foo = true;
            Ok(())
        }
    }

    assert!(!Settings::default().parse(["test"]).unwrap().0.foo);
    assert!(Settings::default().parse(["test", "--foo"]).unwrap().0.foo);
    assert!(Settings::default().parse(["test", "-f"]).unwrap().0.foo);
}

#[test]
fn short_alias() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("-b")]
        Foo,
    }

    #[derive(Default)]
    struct Settings {
        foo: bool,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Foo: Arg) -> Result<(), uutils_args::Error> {
            self.foo = true;
            Ok(())
        }
    }

    assert!(Settings::default().parse(["test", "-b"]).unwrap().0.foo);
}

#[test]
fn long_alias() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("--bar")]
        Foo,
    }

    #[derive(Default)]
    struct Settings {
        foo: bool,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Foo: Arg) -> Result<(), uutils_args::Error> {
            self.foo = true;
            Ok(())
        }
    }

    assert!(Settings::default().parse(["test", "--bar"]).unwrap().0.foo);
}

#[test]
fn short_and_long_alias() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("-b", "--bar")]
        Foo,
        #[arg("-f", "--foo")]
        Bar,
    }

    #[derive(Default, PartialEq, Eq, Debug)]
    struct Settings {
        foo: bool,
        bar: bool,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            match arg {
                Arg::Foo => self.foo = true,
                Arg::Bar => self.bar = true,
            }
            Ok(())
        }
    }

    let foo_true = Settings {
        foo: true,
        bar: false,
    };

    let bar_true = Settings {
        foo: false,
        bar: true,
    };

    assert_eq!(
        Settings::default().parse(["test", "--bar"]).unwrap().0,
        foo_true
    );
    assert_eq!(
        Settings::default().parse(["test", "-b"]).unwrap().0,
        foo_true
    );
    assert_eq!(
        Settings::default().parse(["test", "--foo"]).unwrap().0,
        bar_true
    );
    assert_eq!(
        Settings::default().parse(["test", "-f"]).unwrap().0,
        bar_true
    );
}

#[test]
fn xyz_map_to_abc() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("-x")]
        X,
        #[arg("-y")]
        Y,
        #[arg("-z")]
        Z,
    }

    #[derive(Default, PartialEq, Eq, Debug)]
    struct Settings {
        a: bool,
        b: bool,
        c: bool,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            match arg {
                Arg::X => {
                    self.a = true;
                    self.b = true;
                }
                Arg::Y => {
                    self.b = true;
                    self.c = true;
                }
                Arg::Z => {
                    self.a = true;
                    self.b = true;
                    self.c = true;
                }
            }
            Ok(())
        }
    }

    assert_eq!(
        Settings::default().parse(["test", "-x"]).unwrap().0,
        Settings {
            a: true,
            b: true,
            c: false,
        },
    );

    assert_eq!(
        Settings::default().parse(["test", "-y"]).unwrap().0,
        Settings {
            a: false,
            b: true,
            c: true,
        },
    );

    assert_eq!(
        Settings::default().parse(["test", "-xy"]).unwrap().0,
        Settings {
            a: true,
            b: true,
            c: true,
        },
    );

    assert_eq!(
        Settings::default().parse(["test", "-z"]).unwrap().0,
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
        #[arg("--foo-bar")]
        FooBar,
        #[arg("--super")]
        Super,
    }

    #[derive(Default, PartialEq, Eq, Debug)]
    struct Settings {
        a: bool,
        b: bool,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            match arg {
                Arg::FooBar => self.a = true,
                Arg::Super => self.b = true,
            }
            Ok(())
        }
    }

    assert_eq!(
        Settings::default()
            .parse(["test", "--foo-bar", "--super"])
            .unwrap()
            .0,
        Settings { a: true, b: true }
    )
}

#[test]
fn number_flag() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[arg("-1")]
        One,
    }
    #[derive(Default, PartialEq, Eq, Debug)]
    struct Settings {
        one: bool,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::One: Arg) -> Result<(), uutils_args::Error> {
            self.one = true;
            Ok(())
        }
    }

    assert!(Settings::default().parse(["test", "-1"]).unwrap().0.one)
}

#[test]
fn false_bool() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("-a")]
        A,
        #[arg("-b")]
        B,
    }

    #[derive(Default)]
    struct Settings {
        foo: bool,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            self.foo = match arg {
                Arg::A => true,
                Arg::B => false,
            };
            Ok(())
        }
    }

    assert!(Settings::default().parse(["test", "-a"]).unwrap().0.foo);
    assert!(!Settings::default().parse(["test", "-b"]).unwrap().0.foo);
    assert!(!Settings::default().parse(["test", "-ab"]).unwrap().0.foo);
    assert!(Settings::default().parse(["test", "-ba"]).unwrap().0.foo);
    assert!(
        !Settings::default()
            .parse(["test", "-a", "-b"])
            .unwrap()
            .0
            .foo
    );
    assert!(
        Settings::default()
            .parse(["test", "-b", "-a"])
            .unwrap()
            .0
            .foo
    );
}

#[test]
fn verbosity() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("-v")]
        Verbosity,
    }

    #[derive(Default)]
    struct Settings {
        verbosity: u8,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, Arg::Verbosity: Arg) -> Result<(), uutils_args::Error> {
            self.verbosity += 1;
            Ok(())
        }
    }

    assert_eq!(
        Settings::default()
            .parse(["test", "-v"])
            .unwrap()
            .0
            .verbosity,
        1
    );
    assert_eq!(
        Settings::default()
            .parse(["test", "-vv"])
            .unwrap()
            .0
            .verbosity,
        2
    );
    assert_eq!(
        Settings::default()
            .parse(["test", "-vvv"])
            .unwrap()
            .0
            .verbosity,
        3
    );
}

#[test]
fn infer_long_args() {
    #[derive(Arguments)]
    enum Arg {
        #[arg("--all")]
        All,
        #[arg("--almost-all")]
        AlmostAll,
        #[arg("--author")]
        Author,
    }

    #[derive(Default)]
    struct Settings {
        all: bool,
        almost_all: bool,
        author: bool,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            match arg {
                Arg::All => self.all = true,
                Arg::AlmostAll => self.almost_all = true,
                Arg::Author => self.author = true,
            }
            Ok(())
        }
    }

    assert!(Settings::default().parse(["test", "--all"]).unwrap().0.all);
    assert!(
        Settings::default()
            .parse(["test", "--alm"])
            .unwrap()
            .0
            .almost_all
    );
    assert!(
        Settings::default()
            .parse(["test", "--au"])
            .unwrap()
            .0
            .author
    );
    assert!(Settings::default().parse(["test", "--a"]).is_err());
}

#[test]
fn enum_flag() {
    #[derive(Default, PartialEq, Eq, Debug)]
    enum SomeEnum {
        #[default]
        Foo,
        Bar,
        Baz,
    }

    #[derive(Arguments)]
    enum Arg {
        #[arg("--foo")]
        Foo,
        #[arg("--bar")]
        Bar,
        #[arg("--baz")]
        Baz,
    }

    #[derive(Default)]
    struct Settings {
        foo: SomeEnum,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            self.foo = match arg {
                Arg::Foo => SomeEnum::Foo,
                Arg::Bar => SomeEnum::Bar,
                Arg::Baz => SomeEnum::Baz,
            };
            Ok(())
        }
    }

    assert_eq!(
        Settings::default().parse(["test"]).unwrap().0.foo,
        SomeEnum::Foo
    );
    assert_eq!(
        Settings::default().parse(["test", "--bar"]).unwrap().0.foo,
        SomeEnum::Bar
    );
    assert_eq!(
        Settings::default().parse(["test", "--baz"]).unwrap().0.foo,
        SomeEnum::Baz,
    );
}
