use std::path::PathBuf;

use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
enum Arg {
    #[arg("-d", "--decode")]
    Decode,

    #[arg("-i", "--ignore-garbage")]
    IgnoreGarbage,

    #[arg("-w COLS", "--wrap=COLS")]
    Wrap(usize),

    #[arg("FILE", ..=1)]
    File(PathBuf),
}

struct Settings {
    decode: bool,
    ignore_garbage: bool,
    wrap: Option<usize>,
    file: Option<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            wrap: Some(76),
            decode: Default::default(),
            ignore_garbage: Default::default(),
            file: Default::default(),
        }
    }
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::Decode => self.decode = true,
            Arg::IgnoreGarbage => self.ignore_garbage = true,
            Arg::Wrap(0) => self.wrap = None,
            Arg::Wrap(x) => self.wrap = Some(x),
            Arg::File(f) => self.file = Some(f),
        }
    }
}

#[test]
fn wrap() {
    assert_eq!(Settings::default().parse(["base32"]).wrap, Some(76));
    assert_eq!(Settings::default().parse(["base32", "-w0"]).wrap, None);
    assert_eq!(
        Settings::default().parse(["base32", "-w100"]).wrap,
        Some(100)
    );
    assert_eq!(
        Settings::default().parse(["base32", "--wrap=100"]).wrap,
        Some(100)
    );
}
