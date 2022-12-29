use std::path::PathBuf;

use uutils_args::{Arguments, Options};

#[derive(Default)]
enum NumberingMode {
    #[default]
    None,
    NonEmpty,
    All,
}

#[derive(Clone, Arguments)]
#[help("--help")]
#[version("--version")]
enum Arg {
    #[option("-A", "--show-all")]
    ShowAll,

    #[option("-b", "--number-nonblank")]
    NumberNonblank,

    #[option("-e")]
    ShowNonPrintingEnds,

    #[option("-E")]
    ShowEnds,

    #[option("-n", "--number")]
    Number,

    #[option("-s", "--squeeze-blank")]
    SqueezeBlank,

    #[option("-t")]
    ShowNonPrintingTabs,

    #[option("-T", "--show-tabs")]
    ShowTabs,

    #[option("-v", "--show-nonprinting")]
    ShowNonPrinting,

    #[positional(..)]
    File(PathBuf),
}

#[derive(Default, Options)]
#[arg_type(Arg)]
struct Settings {
    #[map(Arg::ShowAll | Arg::ShowTabs | Arg::ShowNonPrintingTabs => true)]
    show_tabs: bool,

    #[map(Arg::ShowAll | Arg::ShowEnds | Arg::ShowNonPrintingEnds => true)]
    show_ends: bool,

    #[map(
        Arg::ShowAll
        | Arg::ShowNonPrintingEnds
        | Arg::ShowNonPrintingTabs
        | Arg::ShowNonPrinting
            => true
    )]
    show_nonprinting: bool,

    #[map(
        Arg::Number => NumberingMode::All,
        Arg::NumberNonblank => NumberingMode::NonEmpty,
    )]
    number: NumberingMode,

    #[map(Arg::SqueezeBlank => true)]
    squeeze_blank: bool,

    #[collect(set(Arg::File))]
    files: Vec<PathBuf>,
}

#[test]
fn show() {
    let s = Settings::parse(["cat", "-v"]).unwrap();
    assert!(!s.show_ends && !s.show_tabs && s.show_nonprinting);

    let s = Settings::parse(["cat", "-E"]).unwrap();
    assert!(s.show_ends && !s.show_tabs && !s.show_nonprinting);

    let s = Settings::parse(["cat", "-T"]).unwrap();
    assert!(!s.show_ends && s.show_tabs && !s.show_nonprinting);

    let s = Settings::parse(["cat", "-e"]).unwrap();
    assert!(s.show_ends && !s.show_tabs && s.show_nonprinting);

    let s = Settings::parse(["cat", "-t"]).unwrap();
    assert!(!s.show_ends && s.show_tabs && s.show_nonprinting);

    let s = Settings::parse(["cat", "-A"]).unwrap();
    assert!(s.show_ends && s.show_tabs && s.show_nonprinting);

    let s = Settings::parse(["cat", "-te"]).unwrap();
    assert!(s.show_ends && s.show_tabs && s.show_nonprinting);

    let s = Settings::parse(["cat", "-vET"]).unwrap();
    assert!(s.show_ends && s.show_tabs && s.show_nonprinting);
}
