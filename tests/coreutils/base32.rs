use std::path::PathBuf;

use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
#[help("--help")]
#[version("--version")]
enum Arg {
    #[option("-d", "--decode")]
    Decode,

    #[option("-i", "--ignore-garbage")]
    IgnoreGarbage,

    #[option("-w COLS", "--wrap=COLS")]
    Wrap(usize),

    #[positional(..=1)]
    File(PathBuf),
}

#[derive(Options)]
#[arg_type(Arg)]
struct Settings {
    #[map(Arg::Decode => true)]
    decode: bool,

    #[map(Arg::IgnoreGarbage => true)]
    ignore_garbage: bool,

    #[map(
        Arg::Wrap(0) => None,
        Arg::Wrap(n) => Some(n),
    )]
    wrap: Option<usize>,

    #[map(Arg::File(f) => Some(f))]
    file: Option<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            decode: false,
            ignore_garbage: false,
            wrap: Some(76),
            file: None,
        }
    }
}

#[test]
fn wrap() {
    assert_eq!(Settings::parse(["base32"]).unwrap().wrap, Some(76));
    assert_eq!(Settings::parse(["base32", "-w0"]).unwrap().wrap, None);
    assert_eq!(Settings::parse(["base32", "-w100"]).unwrap().wrap, Some(100));
    assert_eq!(Settings::parse(["base32", "--wrap=100"]).unwrap().wrap, Some(100));
}
