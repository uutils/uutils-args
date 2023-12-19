use std::path::{Path, PathBuf};

use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
enum Arg {
    #[arg("-d", "--directory")]
    Directory,

    #[arg("-u", "--dry-run")]
    DryRun,

    #[arg("-q", "--quiet")]
    Quiet,

    #[arg("--suffix=SUFFIX")]
    Suffix(String),

    #[arg("-t")]
    TreatAsTemplate,

    #[arg("-p DIR", "--tmpdir[=DIR]", value = ".".into())]
    TmpDir(PathBuf),
}

#[derive(Default)]
struct Settings {
    directory: bool,
    dry_run: bool,
    quiet: bool,
    tmp_dir: Option<PathBuf>,
    suffix: Option<String>,
    treat_as_template: bool,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::Directory => self.directory = true,
            Arg::DryRun => self.dry_run = true,
            Arg::Quiet => self.quiet = true,
            Arg::Suffix(s) => self.suffix = Some(s),
            Arg::TreatAsTemplate => self.treat_as_template = true,
            Arg::TmpDir(dir) => self.tmp_dir = Some(dir),
        }
    }
}

#[test]
fn suffix() {
    let (s, _operands) = Settings::default().parse(["mktemp", "--suffix=hello"]);
    assert_eq!(s.suffix.unwrap(), "hello");

    let (s, _operands) = Settings::default().parse(["mktemp", "--suffix="]);
    assert_eq!(s.suffix.unwrap(), "");

    let (s, _operands) = Settings::default().parse(["mktemp", "--suffix="]);
    assert_eq!(s.suffix.unwrap(), "");

    let (s, _operands) = Settings::default().parse(["mktemp"]);
    assert_eq!(s.suffix, None);
}

#[test]
fn tmpdir() {
    let (s, _operands) = Settings::default().parse(["mktemp", "--tmpdir"]);
    assert_eq!(s.tmp_dir.unwrap(), Path::new("."));

    let (s, _operands) = Settings::default().parse(["mktemp", "--tmpdir="]);
    assert_eq!(s.tmp_dir.unwrap(), Path::new(""));

    let (s, _operands) = Settings::default().parse(["mktemp", "-p", "foo"]);
    assert_eq!(s.tmp_dir.unwrap(), Path::new("foo"));

    let (s, _operands) = Settings::default().parse(["mktemp", "-pfoo"]);
    assert_eq!(s.tmp_dir.unwrap(), Path::new("foo"));

    let (s, _operands) = Settings::default().parse(["mktemp", "-p", ""]);
    assert_eq!(s.tmp_dir.unwrap(), Path::new(""));

    assert!(Settings::default().try_parse(["mktemp", "-p"]).is_err());
}
