use uutils_args::{Arguments, Initial, Options};

#[test]
fn one_flag() {
    #[derive(Arguments)]
    enum Arg {
        #[option("-f", "--foo")]
        Foo,
    }

    #[derive(Initial)]
    struct Settings {
        foo: bool,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, arg: Self::Arg) {
            match arg {
                Arg::Foo => self.foo = true,
            }
        }
    }

    let settings = Settings::parse(["test", "-f"]);
    assert!(settings.foo);
}

#[test]
fn two_flags() {
    #[derive(Arguments, Clone)]
    enum Arg {
        #[option("-a")]
        A,
        #[option("-b")]
        B,
    }

    #[derive(Initial, PartialEq, Eq, Debug)]
    struct Settings {
        a: bool,
        b: bool,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, arg: Self::Arg) {
            match arg {
                Arg::A => self.a = true,
                Arg::B => self.b = true,
            }
        }
    }

    assert_eq!(
        Settings::parse(["test", "-a"]),
        Settings { a: true, b: false }
    );
    assert_eq!(Settings::parse(["test"]), Settings { a: false, b: false });
    assert_eq!(
        Settings::parse(["test", "-b"]),
        Settings { a: false, b: true }
    );
    assert_eq!(
        Settings::parse(["test", "-a", "-b"]),
        Settings { a: true, b: true }
    );
}

#[test]
fn long_and_short_flag() {
    #[derive(Arguments)]
    enum Arg {
        #[option("-f", "--foo")]
        Foo,
    }

    #[derive(Initial)]
    struct Settings {
        foo: bool,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, Arg::Foo: Self::Arg) {
            self.foo = true;
        }
    }

    assert!(!Settings::parse(["test"]).foo);
    assert!(Settings::parse(["test", "--foo"]).foo);
    assert!(Settings::parse(["test", "-f"]).foo);
}

#[test]
fn short_alias() {
    #[derive(Arguments)]
    enum Arg {
        #[option("-b")]
        Foo,
    }

    #[derive(Initial)]
    struct Settings {
        foo: bool,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, Arg::Foo: Self::Arg) {
            self.foo = true;
        }
    }

    assert!(Settings::parse(["test", "-b"]).foo);
}

#[test]
fn long_alias() {
    #[derive(Arguments)]
    enum Arg {
        #[option("--bar")]
        Foo,
    }

    #[derive(Initial)]
    struct Settings {
        foo: bool,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, Arg::Foo: Self::Arg) {
            self.foo = true;
        }
    }

    assert!(Settings::parse(["test", "--bar"]).foo);
}

#[test]
fn short_and_long_alias() {
    #[derive(Arguments)]
    enum Arg {
        #[option("-b", "--bar")]
        Foo,
        #[option("-f", "--foo")]
        Bar,
    }

    #[derive(Initial, PartialEq, Eq, Debug)]
    struct Settings {
        foo: bool,
        bar: bool,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, arg: Self::Arg) {
            match arg {
                Arg::Foo => self.foo = true,
                Arg::Bar => self.bar = true,
            }
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

    assert_eq!(Settings::parse(["test", "--bar"]), foo_true);
    assert_eq!(Settings::parse(["test", "-b"]), foo_true);
    assert_eq!(Settings::parse(["test", "--foo"]), bar_true);
    assert_eq!(Settings::parse(["test", "-f"]), bar_true);
}

#[test]
fn xyz_map_to_abc() {
    #[derive(Arguments)]
    enum Arg {
        #[option("-x")]
        X,
        #[option("-y")]
        Y,
        #[option("-z")]
        Z,
    }

    #[derive(Initial, PartialEq, Eq, Debug)]
    struct Settings {
        a: bool,
        b: bool,
        c: bool,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, arg: Self::Arg) {
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
        }
    }

    assert_eq!(
        Settings::parse(["test", "-x"]),
        Settings {
            a: true,
            b: true,
            c: false,
        },
    );

    assert_eq!(
        Settings::parse(["test", "-y"]),
        Settings {
            a: false,
            b: true,
            c: true,
        },
    );

    assert_eq!(
        Settings::parse(["test", "-xy"]),
        Settings {
            a: true,
            b: true,
            c: true,
        },
    );

    assert_eq!(
        Settings::parse(["test", "-z"]),
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
        #[option("--foo-bar")]
        FooBar,
        #[option("--super")]
        Super,
    }

    #[derive(Initial, PartialEq, Eq, Debug)]
    struct Settings {
        a: bool,
        b: bool,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, arg: Self::Arg) {
            match arg {
                Arg::FooBar => self.a = true,
                Arg::Super => self.b = true,
            }
        }
    }

    assert_eq!(
        Settings::parse(["test", "--foo-bar", "--super"]),
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
    #[derive(Initial, PartialEq, Eq, Debug)]
    struct Settings {
        one: bool,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, Arg::One: Arg) {
            self.one = true;
        }
    }

    assert!(Settings::parse(["test", "-1"]).one)
}

#[test]
fn false_bool() {
    #[derive(Arguments)]
    enum Arg {
        #[option("-a")]
        A,
        #[option("-b")]
        B,
    }

    #[derive(Initial)]
    struct Settings {
        foo: bool,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, arg: Arg) {
            self.foo = match arg {
                Arg::A => true,
                Arg::B => false,
            }
        }
    }

    assert!(Settings::parse(["test", "-a"]).foo);
    assert!(!Settings::parse(["test", "-b"]).foo);
    assert!(!Settings::parse(["test", "-ab"]).foo);
    assert!(Settings::parse(["test", "-ba"]).foo);
    assert!(!Settings::parse(["test", "-a", "-b"]).foo);
    assert!(Settings::parse(["test", "-b", "-a"]).foo);
}

#[test]
fn verbosity() {
    #[derive(Arguments)]
    enum Arg {
        #[option("-v")]
        Verbosity,
    }

    #[derive(Initial)]
    struct Settings {
        verbosity: u8,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, Arg::Verbosity: Arg) {
            self.verbosity += 1;
        }
    }

    assert_eq!(Settings::parse(["test", "-v"]).verbosity, 1);
    assert_eq!(Settings::parse(["test", "-vv"]).verbosity, 2);
    assert_eq!(Settings::parse(["test", "-vvv"]).verbosity, 3);
}

#[test]
fn infer_long_args() {
    #[derive(Arguments)]
    enum Arg {
        #[option("--all")]
        All,
        #[option("--almost-all")]
        AlmostAll,
        #[option("--author")]
        Author,
    }

    #[derive(Initial)]
    struct Settings {
        all: bool,
        almost_all: bool,
        author: bool,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, arg: Arg) {
            match arg {
                Arg::All => self.all = true,
                Arg::AlmostAll => self.almost_all = true,
                Arg::Author => self.author = true,
            }
        }
    }

    assert!(Settings::parse(["test", "--all"]).all);
    assert!(Settings::parse(["test", "--alm"]).almost_all);
    assert!(Settings::parse(["test", "--au"]).author);
    assert!(Settings::try_parse(["test", "--a"]).is_err());
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
        #[option("--foo")]
        Foo,
        #[option("--bar")]
        Bar,
        #[option("--baz")]
        Baz,
    }

    #[derive(Initial)]
    struct Settings {
        foo: SomeEnum,
    }

    impl Options for Settings {
        type Arg = Arg;
        fn apply(&mut self, arg: Arg) {
            self.foo = match arg {
                Arg::Foo => SomeEnum::Foo,
                Arg::Bar => SomeEnum::Bar,
                Arg::Baz => SomeEnum::Baz,
            };
        }
    }

    assert_eq!(Settings::parse(["test"]).foo, SomeEnum::Foo);
    assert_eq!(Settings::parse(["test", "--bar"]).foo, SomeEnum::Bar);
    assert_eq!(Settings::parse(["test", "--baz"]).foo, SomeEnum::Baz,);
}
