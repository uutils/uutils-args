use std::path::PathBuf;

use uutils_args::{Arguments, Initial, Options};

#[derive(Default)]
enum NumberingMode {
    #[default]
    None,
    NonEmpty,
    All,
}

#[derive(Clone, Arguments)]
enum Arg {
    #[arg("-A", "--show-all")]
    ShowAll,

    #[arg("-b", "--number-nonblank")]
    NumberNonblank,

    #[arg("-e")]
    ShowNonPrintingEnds,

    #[arg("-E")]
    ShowEnds,

    #[arg("-n", "--number")]
    Number,

    #[arg("-s", "--squeeze-blank")]
    SqueezeBlank,

    #[arg("-t")]
    ShowNonPrintingTabs,

    #[arg("-T", "--show-tabs")]
    ShowTabs,

    #[arg("-v", "--show-nonprinting")]
    ShowNonPrinting,

    #[arg("FILES", ..)]
    File(PathBuf),
}

#[derive(Initial)]
struct Settings {
    show_tabs: bool,
    show_ends: bool,
    show_nonprinting: bool,
    number: NumberingMode,
    squeeze_blank: bool,
    files: Vec<PathBuf>,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::ShowAll => {
                self.show_tabs = true;
                self.show_ends = true;
                self.show_nonprinting = true;
            }
            Arg::ShowNonPrintingEnds => {
                self.show_nonprinting = true;
                self.show_ends = true;
            }
            Arg::ShowNonPrintingTabs => {
                self.show_tabs = true;
                self.show_nonprinting = true;
            }
            Arg::ShowEnds => self.show_ends = true,
            Arg::ShowTabs => self.show_tabs = true,
            Arg::ShowNonPrinting => self.show_nonprinting = true,
            Arg::Number => self.number = NumberingMode::All,
            Arg::NumberNonblank => self.number = NumberingMode::NonEmpty,
            Arg::SqueezeBlank => self.squeeze_blank = true,
            Arg::File(f) => self.files.push(f),
        }
    }
}

#[test]
fn show() {
    let s = Settings::parse(["cat", "-v"]);
    assert!(!s.show_ends && !s.show_tabs && s.show_nonprinting);

    let s = Settings::parse(["cat", "-E"]);
    assert!(s.show_ends && !s.show_tabs && !s.show_nonprinting);

    let s = Settings::parse(["cat", "-T"]);
    assert!(!s.show_ends && s.show_tabs && !s.show_nonprinting);

    let s = Settings::parse(["cat", "-e"]);
    assert!(s.show_ends && !s.show_tabs && s.show_nonprinting);

    let s = Settings::parse(["cat", "-t"]);
    assert!(!s.show_ends && s.show_tabs && s.show_nonprinting);

    let s = Settings::parse(["cat", "-A"]);
    assert!(s.show_ends && s.show_tabs && s.show_nonprinting);

    let s = Settings::parse(["cat", "-te"]);
    assert!(s.show_ends && s.show_tabs && s.show_nonprinting);

    let s = Settings::parse(["cat", "-vET"]);
    assert!(s.show_ends && s.show_tabs && s.show_nonprinting);
}
