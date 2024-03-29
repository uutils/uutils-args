use std::ffi::OsString;

use uutils_args::{
    positional::{Opt, Unpack},
    Arguments, Options,
};

#[derive(Clone, Arguments)]
enum Arg {
    #[arg("-d", "--decode")]
    Decode,

    #[arg("-i", "--ignore-garbage")]
    IgnoreGarbage,

    #[arg("-w COLS", "--wrap=COLS")]
    Wrap(usize),
}

struct Settings {
    decode: bool,
    ignore_garbage: bool,
    wrap: Option<usize>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            wrap: Some(76),
            decode: Default::default(),
            ignore_garbage: Default::default(),
        }
    }
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::Decode => self.decode = true,
            Arg::IgnoreGarbage => self.ignore_garbage = true,
            Arg::Wrap(0) => self.wrap = None,
            Arg::Wrap(x) => self.wrap = Some(x),
        }
        Ok(())
    }
}

fn parse<I>(args: I) -> Result<(Settings, Option<OsString>), uutils_args::Error>
where
    I: IntoIterator,
    I::Item: Into<OsString>,
{
    let (s, ops) = Settings::default().parse(args)?;
    let file = Opt("FILE").unpack(ops)?;
    Ok((s, file))
}

#[test]
fn wrap() {
    assert_eq!(parse(["base32"]).unwrap().0.wrap, Some(76));
    assert_eq!(parse(["base32", "-w0"]).unwrap().0.wrap, None);
    assert_eq!(parse(["base32", "-w100"]).unwrap().0.wrap, Some(100));
    assert_eq!(parse(["base32", "--wrap=100"]).unwrap().0.wrap, Some(100));
}

#[test]
fn file() {
    assert_eq!(parse(["base32"]).unwrap().1, None);
    assert_eq!(
        parse(["base32", "file"]).unwrap().1,
        Some(OsString::from("file"))
    );
}
