use std::{ffi::OsString, path::PathBuf};
use uutils_args::{
    positional::{Many0, Opt, Unpack},
    Arguments, Options,
};

#[derive(Clone, Arguments)]
enum Arg {
    #[arg("-e", "--echo")]
    Echo,

    #[arg("-z", "--zero")]
    Zero,
    // Not relevant for this example: -i, -n, -r, -o, --random-source
}

#[derive(Debug, Default, PartialEq)]
struct Settings {
    echo: bool,
    zero: bool,
    echo_args: Vec<OsString>,
    file: Option<PathBuf>,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::Echo => self.echo = true,
            Arg::Zero => self.zero = true,
        }
        Ok(())
    }
}

fn parse(args: &[&str]) -> Result<Settings, uutils_args::Error> {
    let (mut settings, operands) = Settings::default().parse(args)?;

    if settings.echo {
        settings.echo_args = Many0("ARG").unpack(operands)?;
    } else {
        settings.file = Opt("FILE").unpack(operands)?.map(From::<OsString>::from);
    }

    Ok(settings)
}

#[test]
fn noarg_is_file() {
    let settings = parse(&["shuf"]).unwrap();
    assert_eq!(settings, Settings::default());
}

#[test]
fn file_takes_one_arg() {
    let settings = parse(&["shuf", "myfile"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            file: Some("myfile".into()),
            ..Settings::default()
        }
    );
}

#[test]
fn file_refuses_two_files() {
    // FIXME: Check detected error
    assert!(parse(&["shuf", "myfile", "otherfile"]).is_err());
}

#[test]
fn file_refuses_three_files() {
    // FIXME: Check detected error
    assert!(parse(&["shuf", "myfile", "otherfile", "morefile"]).is_err());
}

#[test]
fn noarg_is_file_zero() {
    let settings = parse(&["shuf", "-z"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn the_help() {
    let settings = parse(&["shuf", "--help"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            zero: false,
            ..Settings::default()
        }
    );
}

#[test]
fn file_zero_takes_one_arg() {
    let settings = parse(&["shuf", "-z", "myfile"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            file: Some("myfile".into()),
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn file_zero_postfix_takes_one_arg() {
    let settings = parse(&["shuf", "myfile", "-z"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            file: Some("myfile".into()),
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn file_zero_refuses_two_files() {
    // FIXME: Check detected error
    assert!(parse(&["shuf", "-z", "myfile", "otherfile"]).is_err());
}

#[test]
fn file_zero_refuses_three_files() {
    // FIXME: Check detected error
    assert!(parse(&["shuf", "-z", "myfile", "otherfile", "morefile"]).is_err());
}

#[test]
fn echo_onearg() {
    let settings = parse(&["shuf", "-e", "hello"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into()],
            ..Settings::default()
        }
    );
}

#[test]
fn echo_onearg_postfix() {
    let settings = parse(&["shuf", "hello", "-e"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into()],
            ..Settings::default()
        }
    );
}

#[test]
fn echo_twoarg() {
    let settings = parse(&["shuf", "-e", "hello", "world"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into(), "world".into()],
            ..Settings::default()
        }
    );
}

#[test]
fn echo_twoarg_postfix() {
    let settings = parse(&["shuf", "hello", "world", "-e"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into(), "world".into()],
            ..Settings::default()
        }
    );
}

#[test]
fn echo_twoarg_infix() {
    let settings = parse(&["shuf", "hello", "-e", "world"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into(), "world".into()],
            ..Settings::default()
        }
    );
}

#[test]
fn echo_onearg_zero_before() {
    let settings = parse(&["shuf", "-z", "-e", "hello"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into()],
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_onearg_postfix_zero_before() {
    let settings = parse(&["shuf", "-z", "hello", "-e"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into()],
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_twoarg_zero_before() {
    let settings = parse(&["shuf", "-z", "-e", "hello", "world"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into(), "world".into()],
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_twoarg_postfix_zero_before() {
    let settings = parse(&["shuf", "-z", "hello", "world", "-e"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into(), "world".into()],
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_twoarg_infix_zero_before() {
    let settings = parse(&["shuf", "-z", "hello", "-e", "world"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into(), "world".into()],
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_onearg_zero_after() {
    let settings = parse(&["shuf", "-e", "hello", "-z"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into()],
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_onearg_postfix_zero_after() {
    let settings = parse(&["shuf", "hello", "-e", "-z"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into()],
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_twoarg_zero_after() {
    let settings = parse(&["shuf", "-e", "hello", "world", "-z"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into(), "world".into()],
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_twoarg_postfix_zero_after() {
    let settings = parse(&["shuf", "hello", "world", "-e", "-z"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into(), "world".into()],
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_twoarg_infix_zero_after() {
    let settings = parse(&["shuf", "hello", "-e", "world", "-z"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into(), "world".into()],
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_collapse_zero_before_noarg() {
    let settings = parse(&["shuf", "-ze"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_collapse_zero_before_onearg() {
    let settings = parse(&["shuf", "-ze", "hello"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into()],
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_collapse_zero_after_noarg() {
    let settings = parse(&["shuf", "-ez"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            zero: true,
            ..Settings::default()
        }
    );
}

#[test]
fn echo_collapse_zero_after_onearg() {
    let settings = parse(&["shuf", "-ez", "hello"]).unwrap();
    assert_eq!(
        settings,
        Settings {
            echo: true,
            echo_args: vec!["hello".into()],
            zero: true,
            ..Settings::default()
        }
    );
}
