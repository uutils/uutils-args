use uutils_args::{Arguments, Options};

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
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::Decode => self.decode = true,
            Arg::IgnoreGarbage => self.ignore_garbage = true,
            Arg::Wrap(0) => self.wrap = None,
            Arg::Wrap(x) => self.wrap = Some(x),
        }
    }
}

#[test]
fn wrap() {
    assert_eq!(Settings::default().parse(["base32"]).0.wrap, Some(76));
    assert_eq!(Settings::default().parse(["base32", "-w0"]).0.wrap, None);
    assert_eq!(
        Settings::default().parse(["base32", "-w100"]).0.wrap,
        Some(100)
    );
    assert_eq!(
        Settings::default().parse(["base32", "--wrap=100"]).0.wrap,
        Some(100)
    );
}
