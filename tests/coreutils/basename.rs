use std::ffi::OsString;

use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
enum Arg {
    #[arg("-a", "--multiple")]
    Multiple,

    #[arg("-s SUFFIX", "--suffix=SUFFIX")]
    Suffix(OsString),

    #[arg("-z", "--zero")]
    Zero,
}

#[derive(Default)]
struct Settings {
    multiple: bool,
    suffix: OsString,
    zero: bool,
    names: Vec<OsString>,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::Multiple => self.multiple = true,
            Arg::Suffix(s) => {
                self.multiple = true;
                self.suffix = s
            }
            Arg::Zero => self.zero = true,
        }
    }
}

fn parse(args: &[&str]) -> Settings {
    let (mut settings, operands) = Settings::default().parse(args);
    settings.names = operands;
    if !settings.multiple {
        assert_eq!(settings.names.len(), 2);
        settings.suffix = settings.names.pop().unwrap();
    }
    settings
}

#[test]
fn name_and_suffix() {
    let settings = parse(&["basename", "foobar", "bar"]);
    assert!(!settings.zero);
    assert_eq!(settings.names, vec!["foobar"]);
    assert_eq!(settings.suffix, "bar");
}

#[test]
fn zero_name_and_suffix() {
    let settings = parse(&["basename", "-z", "foobar", "bar"]);
    assert!(settings.zero);
    assert_eq!(settings.names, vec!["foobar"]);
    assert_eq!(settings.suffix, "bar");
}

#[test]
fn all_and_names() {
    let settings = parse(&["basename", "-a", "foobar", "bar"]);
    assert!(settings.multiple);
    assert!(!settings.zero);
    assert_eq!(settings.names, vec!["foobar", "bar"]);
    assert_eq!(settings.suffix, "");
}

#[test]
fn option_like_names() {
    let settings = parse(&["basename", "-a", "--", "-a", "-z", "--suffix=SUFFIX"]);
    assert!(settings.multiple);
    assert!(!settings.zero);
    assert_eq!(settings.names, vec!["-a", "-z", "--suffix=SUFFIX"]);
    assert_eq!(settings.suffix, "");
}
