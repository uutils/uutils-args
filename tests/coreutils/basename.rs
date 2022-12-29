use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
#[help("--help")]
#[version("--version")]
enum Arg {
    #[option("-a", "--multiple")]
    Multiple,

    #[option("-s SUFFIX", "--suffix=SUFFIX")]
    Suffix(String),

    #[option("-z", "--zero")]
    Zero,

    #[positional(last, ..)]
    Names(Vec<String>),
}

#[derive(Default, Options)]
#[arg_type(Arg)]
struct Settings {
    #[map(Arg::Multiple | Arg::Suffix(_) => true)]
    multiple: bool,

    #[set(Arg::Suffix)]
    suffix: String,

    #[map(Arg::Zero => true)]
    zero: bool,

    #[set(Arg::Names)]
    names: Vec<String>,
}

fn parse(args: &'static [&'static str]) -> Result<Settings, uutils_args::Error> {
    let mut settings = Settings::parse(args)?;
    if !settings.multiple {
        assert_eq!(settings.names.len(), 2);
        settings.suffix = settings.names.pop().unwrap();
    }
    Ok(settings)
}

#[test]
fn name_and_suffix() {
    let settings = parse(&["basename", "foobar", "bar"]).unwrap();
    assert!(!settings.zero);
    assert_eq!(settings.names, vec!["foobar"]);
    assert_eq!(settings.suffix, "bar");
}

#[test]
fn zero_name_and_suffix() {
    let settings = parse(&["basename", "-z", "foobar", "bar"]).unwrap();
    assert!(settings.zero);
    assert_eq!(settings.names, vec!["foobar"]);
    assert_eq!(settings.suffix, "bar");
}

#[test]
fn all_and_names() {
    let settings = parse(&["basename", "-a", "foobar", "bar"]).unwrap();
    assert!(settings.multiple);
    assert!(!settings.zero);
    assert_eq!(settings.names, vec!["foobar", "bar"]);
    assert_eq!(settings.suffix, "");
}

#[test]
fn option_like_names() {
    let settings = parse(&["basename", "-a", "--", "-a", "-z", "--suffix=SUFFIX"]).unwrap();
    assert!(settings.multiple);
    assert!(!settings.zero);
    assert_eq!(settings.names, vec!["-a", "-z", "--suffix=SUFFIX"]);
    assert_eq!(settings.suffix, "");
}
