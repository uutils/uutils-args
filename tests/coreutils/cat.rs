use uutils_args::{Arguments, Options};

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
}

#[derive(Default)]
struct Settings {
    show_tabs: bool,
    show_ends: bool,
    show_nonprinting: bool,
    number: NumberingMode,
    squeeze_blank: bool,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
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
        }
        Ok(())
    }
}

#[test]
fn show() {
    let (s, _) = Settings::default().parse(["cat", "-v"]).unwrap();
    assert!(!s.show_ends && !s.show_tabs && s.show_nonprinting);

    let (s, _) = Settings::default().parse(["cat", "-E"]).unwrap();
    assert!(s.show_ends && !s.show_tabs && !s.show_nonprinting);

    let (s, _) = Settings::default().parse(["cat", "-T"]).unwrap();
    assert!(!s.show_ends && s.show_tabs && !s.show_nonprinting);

    let (s, _) = Settings::default().parse(["cat", "-e"]).unwrap();
    assert!(s.show_ends && !s.show_tabs && s.show_nonprinting);

    let (s, _) = Settings::default().parse(["cat", "-t"]).unwrap();
    assert!(!s.show_ends && s.show_tabs && s.show_nonprinting);

    let (s, _) = Settings::default().parse(["cat", "-A"]).unwrap();
    assert!(s.show_ends && s.show_tabs && s.show_nonprinting);

    let (s, _) = Settings::default().parse(["cat", "-te"]).unwrap();
    assert!(s.show_ends && s.show_tabs && s.show_nonprinting);

    let (s, _) = Settings::default().parse(["cat", "-vET"]).unwrap();
    assert!(s.show_ends && s.show_tabs && s.show_nonprinting);
}
