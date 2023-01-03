use std::path::PathBuf;
use uutils_args::{Arguments, FromValue, Options};

#[derive(Clone, Default, Debug, PartialEq, Eq, FromValue)]
enum Format {
    #[value("long")]
    Long,

    #[value("single-column")]
    SingleColumn,

    #[default]
    #[value("columns", "vertical")]
    Columns,

    #[value("across", "horizontal")]
    Across,

    #[value("commas")]
    Commas,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, FromValue)]
enum When {
    #[value("yes", "always", "force")]
    Always,

    #[default]
    #[value("auto", "if-tty", "tty")]
    Auto,

    #[value("no", "never", "none")]
    Never,
}

impl When {
    fn to_bool(&self) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::Auto => atty::is(atty::Stream::Stdout),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
enum Files {
    #[default]
    Default,
    AlmostAll,
    All,
}

#[derive(Default, Debug, PartialEq, Eq)]
enum Dereference {
    // None,
    #[default]
    DirArgs,
    Args,
    All,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, FromValue)]
enum QuotingStyle {
    #[value("literal")]
    Literal,

    #[default]
    #[value("shell")]
    Shell,

    #[value("shell-always")]
    ShellAlways,

    #[value("shell-escape")]
    ShellEscape,

    #[value("shell-escape-always")]
    ShellEscapeAlways,

    #[value("c")]
    C,

    #[value("escape")]
    Escape,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, FromValue)]
enum Sort {
    #[default]
    Name,
    #[value("none")]
    None,
    #[value("size")]
    Size,
    #[value("time")]
    Time,
    #[value("version")]
    Version,
    #[value("extension")]
    Extension,
    #[value("width")]
    Width,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, FromValue)]
enum Time {
    #[default]
    Modification,
    #[value("access", "atime", "use")]
    Access,
    #[value("change", "ctime", "status")]
    Change,
    #[value("birth", "creation")]
    Birth,
}

#[derive(Clone, Default, Debug, FromValue, PartialEq, Eq)]
enum IndicatorStyle {
    #[default]
    #[value("none")]
    None,
    #[value("slash")]
    Slash,
    #[value("file-type")]
    FileType,
    #[value("classify")]
    Classify,
}

#[derive(Clone, Arguments)]
enum Arg {
    // === Files ===
    /// Do not ignore entries starting with .
    #[option("-a")]
    All,

    /// Do not list implied . and ..
    #[option("-A")]
    AlmostAll,

    /// Show file author (ignored)
    #[option("--author")]
    Author,

    // === Time ===
    #[option("-c")]
    ChangeTime,

    #[option("-u")]
    AccessTime,

    #[option("--time=WORD")]
    Time(Time),

    // === Sorting ===
    #[option("--sort=WORD")]
    Sort(Sort),

    #[option("-t")]
    SortTime,

    #[option("-U")]
    SortNone,

    #[option("-v")]
    SortVersion,

    #[option("-X")]
    SortExtension,

    // === Miscellaneous ===
    #[option("-Z", "--context")]
    SecurityContext,

    /// Do not list files starting with ~
    #[option("-B", "--ignore-backups")]
    IgnoreBackups,

    #[option("-d", "--directory")]
    Directory,

    #[option("-D", "--dired")]
    Dired,

    #[option("--hyperlink")]
    Hyperlink(When),

    #[option("-i", "--inode")]
    Inode,

    #[option("-I PATTERN", "--ignore=PATTERN")]
    Ignore(String),

    #[option("-r", "--reverse")]
    Reverse,

    #[option("-R", "--recursive")]
    Recursive,

    #[option("-w COLS", "--width=COLS")]
    Width(u16),

    #[option("-s", "--size")]
    AllocationSize,

    #[option("-G", "--no-group")]
    NoGroup,

    // === Format ===
    /// Set long format
    #[option("-l", "--long")]
    Long,

    /// Set columns format
    #[option("-C")]
    Columns,

    /// Set across format
    #[option("-x")]
    Across,

    /// Set comma format
    #[option("-m")]
    Commas,

    /// Show single column
    #[option("-1")]
    SingleColumn,

    #[option("-o")]
    LongNoGroup,

    #[option("-g")]
    LongNoOwner,

    #[option("-n", "--numeric-uid-gid")]
    LongNumericUidGid,

    /// Set format
    #[option("--format=FORMAT")]
    Format(Format),

    // === Indicator style ===
    #[option("--indicator-style=STYLE")]
    IndicatorStyle(IndicatorStyle),

    #[option("-p")]
    IndicatorStyleSlash,

    #[option("--file-type")]
    IndicatorStyleFileType,

    #[option("-F", "--classify[=WHEN]", default = When::Always)]
    IndicatorStyleClassify(When),

    // === Dereference ===
    #[option("-L", "--dereference")]
    DerefAll,

    #[option("--dereference-command-line-symlink-to-dir")]
    DerefDirArgs,

    #[option("--dereference-command-line")]
    DerefArgs,

    // === Size ===
    #[option("-h", "--human-readable")]
    HumanReadable,

    #[option("-k", "--kibibytes")]
    Kibibytes,

    #[option("--si")]
    Si,

    // #[option("--block-size=BLOCKSIZE")]
    // BlockSize(Size),

    // === Quoting style ===
    #[option("--quoting-style=STYLE")]
    QuotingStyle(QuotingStyle),

    #[option("-N", "--literal")]
    Literal,

    #[option("-h", "--escape")]
    Escape,

    #[option("-Q", "--quote-name")]
    QuoteName,

    /// Set the color
    #[option("--color[=WHEN]", default = When::Always)]
    Color(When),

    /// Print control characters as ?
    #[option("-q", "--hide-control-chars")]
    HideControlChars,

    /// Show control characters as is
    #[option("--show-control-chars")]
    ShowControlChars,

    #[option("--zero")]
    Zero,

    #[option("--group-directories-first")]
    GroupDirectoriesFirst,

    #[positional(..)]
    File(PathBuf),
}

fn default_terminal_size() -> u16 {
    if let Some((width, _)) = terminal_size::terminal_size() {
        return width.0;
    }

    if let Some(columns) = std::env::var_os("COLUMNS") {
        match columns.to_str().and_then(|s| s.parse().ok()) {
            Some(columns) => return columns,
            None => {
                // TODO: Make show_error! when integrated with uutils
                println!(
                    "ignoring invalid width in environment variable COLUMNS: '{}'",
                    columns.to_string_lossy()
                );
            }
        }
    }

    80
}

#[derive(Default, Options, Debug, PartialEq, Eq)]
#[arg_type(Arg)]
struct Settings {
    #[map(
        Arg::Long | Arg::LongNoGroup | Arg::LongNoOwner | Arg::LongNumericUidGid => Format::Long,
        Arg::Columns => Format::Columns,
        Arg::Across => Format::Across,
        Arg::Commas => Format::Commas,
        Arg::SingleColumn => Format::SingleColumn,
        Arg::Format(f) => f,
    )]
    format: Format,

    #[collect(set(Arg::File))]
    files: Vec<PathBuf>,

    #[map(
        Arg::Sort(s) => s,
        Arg::SortTime => Sort::Time,
        Arg::SortNone => Sort::None,
        Arg::SortVersion => Sort::Version,
        Arg::SortExtension => Sort::Extension,
    )]
    sort: Sort,

    #[map(Arg::Recursive => true)]
    recursive: bool,

    #[map(Arg::Reverse => true)]
    reverse: bool,

    #[map(
        Arg::DerefAll => Dereference::All,
        Arg::DerefDirArgs => Dereference::DirArgs,
        Arg::DerefArgs => Dereference::Args,
    )]
    dereference: Dereference,

    #[collect(set(Arg::Ignore))]
    ignore_patterns: Vec<String>,
    //
    // size_format: SizeFormat,
    //
    #[map(Arg::Directory => true)]
    directory: bool,

    #[map(
        Arg::ChangeTime => Time::Change,
        Arg::AccessTime => Time::Access,
        Arg::Time(t) => t,
    )]
    time: Time,

    #[map(Arg::Inode => true)]
    inode: bool,

    #[map(Arg::Color(when) => when.to_bool())]
    color: bool,

    #[map(Arg::Author => true)]
    long_author: bool,

    #[map(Arg::LongNoGroup => true)]
    long_no_group: bool,

    #[map(Arg::LongNoOwner => true)]
    long_no_owner: bool,

    #[map(Arg::LongNumericUidGid => true)]
    long_numeric_uid_gid: bool,

    // alloc_size: bool,

    // block_size: Option<u64>,
    #[set(Arg::Width)]
    #[field(default = default_terminal_size())]
    width: u16,

    #[map(
        Arg::QuotingStyle(q) => q,
        Arg::Literal => QuotingStyle::Literal,
        Arg::Escape => QuotingStyle::Escape,
    )]
    quoting_style: QuotingStyle,

    #[map(
        Arg::IndicatorStyleClassify(when) => {
            if when.to_bool() {
                IndicatorStyle::Classify
            } else {
                IndicatorStyle::None
            }
        }
        Arg::IndicatorStyle(style) => style,
        Arg::IndicatorStyleSlash => IndicatorStyle::Slash,
        Arg::IndicatorStyleFileType => IndicatorStyle::FileType,
    )]
    indicator_style: IndicatorStyle,

    // TODO for the full implementation, to complicated
    // to do here.
    // time_style: TimeStyle,
    //
    #[map(Arg::SecurityContext => true)]
    context: bool,

    #[map(Arg::GroupDirectoriesFirst => true)]
    group_directories_first: bool,

    #[map(Arg::Zero => '\0')]
    #[field(default = '\n')]
    eol: char,

    #[map(
        Arg::AlmostAll => Files::AlmostAll,
        Arg::All => Files::All,
    )]
    which_files: Files,

    #[map(Arg::IgnoreBackups => true)]
    ignore_backups: bool,

    #[map(
        Arg::HideControlChars => true,
        Arg::ShowControlChars => false,
    )]
    hide_control_chars: bool,
}

#[test]
fn default() {
    assert_eq!(
        Settings::parse(["ls"]),
        Settings {
            format: Format::Columns,
            files: Vec::new(),
            sort: Sort::Name,
            recursive: false,
            reverse: false,
            dereference: Dereference::DirArgs,
            directory: false,
            time: Time::Modification,
            inode: false,
            color: false,
            long_author: false,
            long_no_group: false,
            long_no_owner: false,
            long_numeric_uid_gid: false,
            width: if let Some((width, _)) = terminal_size::terminal_size() {
                width.0
            } else {
                80
            },
            quoting_style: QuotingStyle::Shell,
            indicator_style: IndicatorStyle::None,
            ignore_patterns: Vec::new(),
            context: false,
            group_directories_first: false,
            eol: '\n',
            which_files: Files::Default,
            ignore_backups: false,
            hide_control_chars: false,
        }
    );
}

#[test]
fn color() {
    let s = Settings::parse(["ls", "--color"]);
    assert!(s.color);

    let s = Settings::parse(["ls", "--color=always"]);
    assert!(s.color);

    let s = Settings::parse(["ls", "--color=never"]);
    assert!(!s.color);
}

#[test]
fn format() {
    let s = Settings::parse(["ls", "-l"]);
    assert_eq!(s.format, Format::Long);

    let s = Settings::parse(["ls", "-m"]);
    assert_eq!(s.format, Format::Commas);

    let s = Settings::parse(["ls", "--format=across"]);
    assert_eq!(s.format, Format::Across);

    let s = Settings::parse(["ls", "--format=acr"]);
    assert_eq!(s.format, Format::Across);

    let s = Settings::parse(["ls", "-o"]);
    assert_eq!(s.format, Format::Long);
    assert!(s.long_no_group && !s.long_no_owner && !s.long_numeric_uid_gid);

    let s = Settings::parse(["ls", "-g"]);
    assert_eq!(s.format, Format::Long);
    assert!(!s.long_no_group && s.long_no_owner && !s.long_numeric_uid_gid);

    let s = Settings::parse(["ls", "-n"]);
    assert_eq!(s.format, Format::Long);
    assert!(!s.long_no_group && !s.long_no_owner && s.long_numeric_uid_gid);

    let s = Settings::parse(["ls", "-og"]);
    assert_eq!(s.format, Format::Long);
    assert!(s.long_no_group && s.long_no_owner && !s.long_numeric_uid_gid);

    let s = Settings::parse(["ls", "-on"]);
    assert_eq!(s.format, Format::Long);
    assert!(s.long_no_group && !s.long_no_owner && s.long_numeric_uid_gid);

    let s = Settings::parse(["ls", "-onCl"]);
    assert_eq!(s.format, Format::Long);
    assert!(s.long_no_group && !s.long_no_owner && s.long_numeric_uid_gid);
}

#[test]
fn time() {
    let s = Settings::parse(["ls", "--time=access"]);
    assert_eq!(s.time, Time::Access);

    let s = Settings::parse(["ls", "--time=a"]);
    assert_eq!(s.time, Time::Access);
}

#[test]
fn classify() {
    let s = Settings::parse(["ls", "--indicator-style=classify"]);
    assert_eq!(s.indicator_style, IndicatorStyle::Classify);

    let s = Settings::parse(["ls", "--classify"]);
    assert_eq!(s.indicator_style, IndicatorStyle::Classify);

    let s = Settings::parse(["ls", "--classify=always"]);
    assert_eq!(s.indicator_style, IndicatorStyle::Classify);

    let s = Settings::parse(["ls", "--classify=none"]);
    assert_eq!(s.indicator_style, IndicatorStyle::None);

    let s = Settings::parse(["ls", "-F"]);
    assert_eq!(s.indicator_style, IndicatorStyle::Classify);
}
