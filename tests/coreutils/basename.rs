use std::ffi::OsString;

use uutils_args::{
    positional::{Many1, Unpack},
    Arguments, Options,
};

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
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::Multiple => self.multiple = true,
            Arg::Suffix(s) => {
                self.multiple = true;
                self.suffix = s
            }
            Arg::Zero => self.zero = true,
        }
        Ok(())
    }
}

fn parse(args: &[&str]) -> Result<Settings, uutils_args::Error> {
    let (mut settings, operands) = Settings::default().parse(args)?;

    if settings.multiple {
        let names = Many1("FILE").unpack(operands)?;
        settings.names = names;
    } else {
        let (names, suffix) = ("FILE", "SUFFIX").unpack(operands)?;
        settings.names = vec![names];
        settings.suffix = suffix;
    }

    Ok(settings)
}

#[test]
fn name_and_suffix() {
    let settings = parse(&["basename", "foobar", "bar"]).unwrap();
    assert!(!settings.zero);
    assert_eq!(settings.names, vec!["foobar"]);
    assert_eq!(settings.suffix, "bar");
}

#[test]
fn zero_name_and_suffix() {
    let settings = parse(&["basename", "-z", "foobar", "bar"]).unwrap();
    assert!(settings.zero);
    assert_eq!(settings.names, vec!["foobar"]);
    assert_eq!(settings.suffix, "bar");
}

#[test]
fn all_and_names() {
    let settings = parse(&["basename", "-a", "foobar", "bar"]).unwrap();
    assert!(settings.multiple);
    assert!(!settings.zero);
    assert_eq!(settings.names, vec!["foobar", "bar"]);
    assert_eq!(settings.suffix, "");
}

#[test]
fn option_like_names() {
    let settings = parse(&["basename", "-a", "--", "-a", "-z", "--suffix=SUFFIX"]).unwrap();
    assert!(settings.multiple);
    assert!(!settings.zero);
    assert_eq!(settings.names, vec!["-a", "-z", "--suffix=SUFFIX"]);
    assert_eq!(settings.suffix, "");
}
