use std::ffi::OsString;
use uutils_args::{Arguments, Options};

#[derive(Debug, Clone, Arguments)]
enum Arg {
    #[arg("-p PREFIX", "--prefix=PREFIX")]
    Prefix(String),
}

#[derive(Default, Debug, PartialEq)]
struct Settings {
    prefix: Option<String>,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::Prefix(prefix) => {
                self.prefix = Some(prefix);
            }
        }
        Ok(())
    }
}

// Convenience function for testing
#[cfg(test)]
impl Settings {
    fn of(prefix: &str) -> Settings {
        Settings {
            prefix: Some(prefix.to_owned()),
        }
    }
}

#[test]
fn prefix_none() {
    assert_eq!(
        Settings::default().parse::<[OsString; 0]>([]).unwrap(),
        (Settings::default(), vec![]),
    );
    assert_eq!(
        Settings::default().parse(["bin_name"]).unwrap(),
        (Settings::default(), vec![]),
    );
    assert_eq!(
        Settings::default().parse(["bin_name", "filename"]).unwrap(),
        (Settings::default(), vec![OsString::from("filename")]),
    );
}

#[test]
fn prefix_short() {
    assert_eq!(
        Settings::default()
            .parse(["bin_name", "-p=", "something"])
            .unwrap(),
        (Settings::of(""), vec![OsString::from("something")]),
    );
    assert_eq!(
        Settings::default()
            .parse(["bin_name", "-p", "", "something"])
            .unwrap(),
        (Settings::of(""), vec![OsString::from("something")]),
    );
    assert_eq!(
        Settings::default()
            .parse(["bin_name", "-p", "something"])
            .unwrap(),
        (Settings::of("something"), vec![]),
    );
    assert_eq!(
        Settings::default()
            .parse(["bin_name", "-p=something"])
            .unwrap(),
        (Settings::of("something"), vec![]),
    );
}

#[test]
fn prefix_long() {
    assert_eq!(
        Settings::default()
            .parse(["bin_name", "--prefix=", "something"])
            .unwrap(),
        (Settings::of(""), vec![OsString::from("something")]),
    );
    assert_eq!(
        Settings::default()
            .parse(["bin_name", "--prefix", "", "something"])
            .unwrap(),
        (Settings::of(""), vec![OsString::from("something")]),
    );
    assert_eq!(
        Settings::default()
            .parse(["bin_name", "--prefix", "something"])
            .unwrap(),
        (Settings::of("something"), vec![]),
    );
    assert_eq!(
        Settings::default()
            .parse(["bin_name", "--prefix=something"])
            .unwrap(),
        (Settings::of("something"), vec![]),
    );
}
