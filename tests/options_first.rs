use std::ffi::OsString;

use uutils_args::{Arguments, Options};

#[test]
fn timeout_like() {
    // The timeout coreutil has -v and a command argument
    #[derive(Arguments)]
    #[arguments(options_first)]
    enum Arg {
        #[arg("-v", "--verbose")]
        Verbose,
    }

    #[derive(Default)]
    struct Settings {
        verbose: bool,
    }

    impl Options<Arg> for Settings {
        fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
            match arg {
                Arg::Verbose => self.verbose = true,
            }
            Ok(())
        }
    }

    let (settings, command) = Settings::default()
        .parse(["timeout", "-v", "10", "foo", "-v"])
        .unwrap();

    assert!(settings.verbose);
    assert_eq!(
        command,
        vec![
            OsString::from("10"),
            OsString::from("foo"),
            OsString::from("-v")
        ]
    );

    let (settings, command) = Settings::default()
        .parse(["timeout", "10", "foo", "-v"])
        .unwrap();

    assert!(!settings.verbose);
    assert_eq!(
        command,
        vec![
            OsString::from("10"),
            OsString::from("foo"),
            OsString::from("-v")
        ]
    );

    let (settings, command) = Settings::default()
        .parse(["timeout", "--", "10", "-v"])
        .unwrap();

    assert!(!settings.verbose);
    assert_eq!(command, vec![OsString::from("10"), OsString::from("-v")]);
}
