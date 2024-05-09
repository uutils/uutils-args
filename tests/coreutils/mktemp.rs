use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use uutils_args::{
    positional::{Opt, Unpack},
    Arguments, Options,
};

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
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::Directory => self.directory = true,
            Arg::DryRun => self.dry_run = true,
            Arg::Quiet => self.quiet = true,
            Arg::Suffix(s) => self.suffix = Some(s),
            Arg::TreatAsTemplate => self.treat_as_template = true,
            Arg::TmpDir(dir) => self.tmp_dir = Some(dir),
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
fn suffix() {
    let (s, _template) = parse(["mktemp", "--suffix=hello"]).unwrap();
    assert_eq!(s.suffix.unwrap(), "hello");

    let (s, _template) = parse(["mktemp", "--suffix="]).unwrap();
    assert_eq!(s.suffix.unwrap(), "");

    let (s, _template) = parse(["mktemp", "--suffix="]).unwrap();
    assert_eq!(s.suffix.unwrap(), "");

    let (s, _template) = parse(["mktemp"]).unwrap();
    assert_eq!(s.suffix, None);
}

#[test]
fn tmpdir() {
    let (s, _template) = parse(["mktemp", "--tmpdir"]).unwrap();
    assert_eq!(s.tmp_dir.unwrap(), Path::new("."));

    let (s, _template) = parse(["mktemp", "--tmpdir="]).unwrap();
    assert_eq!(s.tmp_dir.unwrap(), Path::new(""));

    let (s, _template) = parse(["mktemp", "-p", "foo"]).unwrap();
    assert_eq!(s.tmp_dir.unwrap(), Path::new("foo"));

    let (s, _template) = parse(["mktemp", "-pfoo"]).unwrap();
    assert_eq!(s.tmp_dir.unwrap(), Path::new("foo"));

    let (s, _template) = parse(["mktemp", "-p", ""]).unwrap();
    assert_eq!(s.tmp_dir.unwrap(), Path::new(""));

    assert!(parse(["mktemp", "-p"]).is_err());
}

#[test]
fn too_many_arguments() {
    assert!(parse(["mktemp", "foo", "bar"]).is_err());
}
