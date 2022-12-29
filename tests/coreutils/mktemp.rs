use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
#[help("--help")]
#[version("--version")]
enum Arg {
    #[option("-d", "--directory")]
    Directory,

    #[option("-u", "--dry-run")]
    DryRun,

    #[option("-q", "--quiet")]
    Quiet,

    #[option("--suffix")]
    Suffix(String),

    #[option("-t")]
    TreatAsTemplate,

    #[option("-p", "--tmpdir")]
    TmpDir(Option<String>),

    #[positional(0..=1)]
    Template(String),
}

#[derive(Default, Options)]
#[arg_type(Arg)]
struct Settings {
    #[map(Arg::Directory => true)]
    directory: bool,

    #[map(Arg::DryRun => true)]
    dry_run: bool,

    #[map(Arg::Quiet => true)]
    quiet: bool,

    #[map(Arg::Suffix(s) => Some(s))]
    suffix: Option<String>,

    #[map(Arg::TreatAsTemplate => true)]
    treat_as_template: bool,

    #[set(Arg::Template)]
    template: String,
}

#[test]
fn test() {
    assert!(true);
}
