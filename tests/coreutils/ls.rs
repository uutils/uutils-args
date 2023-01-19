use std::path::PathBuf;
use uutils_args::{Arguments, FromValue, Initial, Options};

#[derive(Default, Debug, PartialEq, Eq, FromValue)]
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

#[derive(Default, Debug, PartialEq, Eq, FromValue)]
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

#[derive(Default, Debug, PartialEq, Eq, FromValue)]
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

#[derive(Default, Debug, PartialEq, Eq, FromValue)]
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

#[derive(Default, Debug, PartialEq, Eq, FromValue)]
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

#[derive(Default, Debug, FromValue, PartialEq, Eq)]
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

#[derive(Arguments)]
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

#[derive(Initial, Debug, PartialEq, Eq)]
struct Settings {
    format: Format,
    files: Vec<PathBuf>,
    sort: Sort,
    recursive: bool,
    reverse: bool,
    dereference: Dereference,
    ignore_patterns: Vec<String>,
    // size_format: SizeFormat,
    directory: bool,
    time: Time,
    inode: bool,
    color: bool,
    long_author: bool,
    long_no_group: bool,
    long_no_owner: bool,
    long_numeric_uid_gid: bool,
    // alloc_size: bool,
    // block_size: Option<u64>,
    #[field(default = default_terminal_size())]
    width: u16,
    quoting_style: QuotingStyle,
    indicator_style: IndicatorStyle,
    // time_style: TimeStyle,
    context: bool,
    group_directories_first: bool,
    #[field(default = '\n')]
    eol: char,
    which_files: Files,
    ignore_backups: bool,
    hide_control_chars: bool,
}

impl Options for Settings {
    type Arg = Arg;
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::All => self.which_files = Files::All,
            Arg::AlmostAll => self.which_files = Files::AlmostAll,
            Arg::Author => self.long_author = true,
            Arg::ChangeTime => self.time = Time::Change,
            Arg::AccessTime => self.time = Time::Access,
            Arg::Time(t) => self.time = t,
            Arg::Sort(s) => self.sort = s,
            Arg::SortTime => self.sort = Sort::Time,
            Arg::SortNone => self.sort = Sort::None,
            Arg::SortVersion => self.sort = Sort::Version,
            Arg::SortExtension => self.sort = Sort::Extension,
            Arg::SecurityContext => self.context = true,
            Arg::IgnoreBackups => self.ignore_backups = true,
            Arg::Directory => self.directory = true,
            Arg::Dired => todo!(),
            Arg::Hyperlink(_when) => todo!(),
            Arg::Inode => self.inode = true,
            Arg::Ignore(pattern) => self.ignore_patterns.push(pattern),
            Arg::Reverse => self.reverse = true,
            Arg::Recursive => self.recursive = true,
            Arg::Width(w) => self.width = w,
            Arg::AllocationSize => todo!(),
            Arg::NoGroup => self.long_no_group = true,
            Arg::Long => self.format = Format::Long,
            Arg::Columns => self.format = Format::Columns,
            Arg::Across => self.format = Format::Across,
            Arg::Commas => self.format = Format::Commas,
            Arg::SingleColumn => self.format = Format::SingleColumn,
            Arg::LongNoGroup => {
                self.format = Format::Long;
                self.long_no_group = true;
            }
            Arg::LongNoOwner => {
                self.format = Format::Long;
                self.long_no_owner = true;
            }
            Arg::LongNumericUidGid => {
                self.format = Format::Long;
                self.long_numeric_uid_gid = true;
            }
            Arg::Format(f) => self.format = f,
            Arg::IndicatorStyle(style) => self.indicator_style = style,
            Arg::IndicatorStyleSlash => self.indicator_style = IndicatorStyle::Slash,
            Arg::IndicatorStyleFileType => self.indicator_style = IndicatorStyle::FileType,
            Arg::IndicatorStyleClassify(when) => {
                self.indicator_style = if when.to_bool() {
                    IndicatorStyle::Classify
                } else {
                    IndicatorStyle::None
                }
            }
            Arg::DerefAll => self.dereference = Dereference::All,
            Arg::DerefDirArgs => self.dereference = Dereference::DirArgs,
            Arg::DerefArgs => self.dereference = Dereference::Args,
            Arg::HumanReadable => todo!(),
            Arg::Kibibytes => todo!(),
            Arg::Si => todo!(),
            Arg::QuotingStyle(style) => self.quoting_style = style,
            Arg::Literal => self.quoting_style = QuotingStyle::Literal,
            Arg::Escape => self.quoting_style = QuotingStyle::Escape,
            Arg::QuoteName => todo!(),
            Arg::Color(when) => self.color = when.to_bool(),
            Arg::HideControlChars => self.hide_control_chars = true,
            Arg::ShowControlChars => self.hide_control_chars = false,
            Arg::Zero => {
                self.eol = '\0';
                // TODO: Zero changes more than just this
            }
            Arg::GroupDirectoriesFirst => self.group_directories_first = true,
            Arg::File(f) => self.files.push(f),
        }
    }
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
