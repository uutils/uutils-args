use uutils_args::{Arguments, Options, Value};

#[derive(Default, Debug, PartialEq, Eq, Value)]
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

#[derive(Default, Debug, PartialEq, Eq, Value)]
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
            // Should be atty::is(atty::Stream::Stdout), but I don't want to
            // pull that dependency in just for this test.
            Self::Auto => true,
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

#[derive(Default, Debug, PartialEq, Eq, Value)]
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

#[derive(Default, Debug, PartialEq, Eq, Value)]
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

#[derive(Default, Debug, PartialEq, Eq, Value)]
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

#[derive(Default, Debug, Value, PartialEq, Eq)]
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
    #[arg("-a")]
    All,

    /// Do not list implied . and ..
    #[arg("-A")]
    AlmostAll,

    /// Show file author (ignored)
    #[arg("--author")]
    Author,

    #[arg("--time=WORD")]
    #[arg("-c", value = Time::Change)]
    #[arg("-u", value = Time::Access)]
    Time(Time),

    // === Sorting ==
    /// Sort by WORD
    #[arg("--sort=WORD")]
    #[arg("-t", value = Sort::Time, help = "Sort by time")]
    #[arg("-U", value = Sort::None, help = "Do not sort")]
    #[arg("-v", value = Sort::Version, help = "Sort by version")]
    #[arg("-X", value = Sort::Extension, help = "Sort by extension")]
    Sort(Sort),

    // === Miscellaneous ===
    #[arg("-Z", "--context")]
    SecurityContext,

    /// Do not list files starting with ~
    #[arg("-B", "--ignore-backups")]
    IgnoreBackups,

    #[arg("-d", "--directory")]
    Directory,

    #[arg("-D", "--dired")]
    Dired,

    #[arg("--hyperlink")]
    Hyperlink(When),

    #[arg("-i", "--inode")]
    Inode,

    #[arg("-I PATTERN", "--ignore=PATTERN")]
    Ignore(String),

    #[arg("-r", "--reverse")]
    Reverse,

    #[arg("-R", "--recursive")]
    Recursive,

    #[arg("-w COLS", "--width=COLS")]
    Width(u16),

    #[arg("-s", "--size")]
    AllocationSize,

    #[arg("-G", "--no-group")]
    NoGroup,

    // === Format ===
    /// Set format
    #[arg("--format=FORMAT")]
    #[arg("-l", "--long", value = Format::Long, help = "Use long format")]
    #[arg("-C", value = Format::Columns, help = "Use columns format")]
    #[arg("-x", value = Format::Across, help = "Use across format")]
    #[arg("-m", value = Format::Commas, help = "Use comma format")]
    Format(Format),

    /// Show single column
    #[arg("-1")]
    SingleColumn,

    #[arg("-o")]
    LongNoGroup,

    #[arg("-g")]
    LongNoOwner,

    #[arg("-n", "--numeric-uid-gid")]
    LongNumericUidGid,

    // === Indicator style ===
    #[arg("--indicator-style=STYLE")]
    #[arg("-p", value = IndicatorStyle::Slash, help = "Append slash to directories")]
    #[arg("--file-type", value = IndicatorStyle::FileType, help = "Add indicators for file types")]
    IndicatorStyle(IndicatorStyle),

    /// Classify items
    #[arg("-F", "--classify[=WHEN]", value = When::Always)]
    IndicatorStyleClassify(When),

    // === Dereference ===
    #[arg("-L", "--dereference")]
    DerefAll,

    #[arg("--dereference-command-line-symlink-to-dir")]
    DerefDirArgs,

    #[arg("--dereference-command-line")]
    DerefArgs,

    // === Size ===
    #[arg("-h", "--human-readable")]
    HumanReadable,

    #[arg("-k", "--kibibytes")]
    Kibibytes,

    #[arg("--si")]
    Si,

    // #[arg("--block-size=BLOCKSIZE")]
    // BlockSize(Size),

    // === Quoting style ===
    #[arg("--quoting-style=STYLE")]
    #[arg("-N", "--literal", value = QuotingStyle::Literal)]
    #[arg("-h", "--escape", value = QuotingStyle::Escape)]
    #[arg("-Q", "--quote-name", value = todo!())]
    QuotingStyle(QuotingStyle),

    /// Set the color
    #[arg("--color[=WHEN]", value = When::Always)]
    Color(When),

    /// Print control characters as ?
    #[arg("-q", "--hide-control-chars")]
    HideControlChars,

    /// Show control characters as is
    #[arg("--show-control-chars")]
    ShowControlChars,

    #[arg("--zero")]
    Zero,

    #[arg("--group-directories-first")]
    GroupDirectoriesFirst,
}

fn default_terminal_size() -> u16 {
    // There should be a check for the terminal size here, but that requires
    // additional dependencies. Besides, it would make the tests dependent on
    // the terminal width, which is not great.

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

#[derive(Debug, PartialEq, Eq)]
struct Settings {
    format: Format,
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
    width: u16,
    quoting_style: QuotingStyle,
    indicator_style: IndicatorStyle,
    // time_style: TimeStyle,
    context: bool,
    group_directories_first: bool,
    eol: char,
    which_files: Files,
    ignore_backups: bool,
    hide_control_chars: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            eol: '\n',
            width: default_terminal_size(),
            format: Default::default(),
            sort: Default::default(),
            recursive: Default::default(),
            reverse: Default::default(),
            dereference: Default::default(),
            ignore_patterns: Default::default(),
            directory: Default::default(),
            time: Default::default(),
            inode: Default::default(),
            color: Default::default(),
            long_author: Default::default(),
            long_no_group: Default::default(),
            long_no_owner: Default::default(),
            long_numeric_uid_gid: Default::default(),
            quoting_style: Default::default(),
            indicator_style: Default::default(),
            context: Default::default(),
            group_directories_first: Default::default(),
            which_files: Default::default(),
            ignore_backups: Default::default(),
            hide_control_chars: Default::default(),
        }
    }
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::All => self.which_files = Files::All,
            Arg::AlmostAll => self.which_files = Files::AlmostAll,
            Arg::Author => self.long_author = true,
            Arg::Time(t) => self.time = t,
            Arg::Sort(s) => self.sort = s,
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
            Arg::Color(when) => self.color = when.to_bool(),
            Arg::HideControlChars => self.hide_control_chars = true,
            Arg::ShowControlChars => self.hide_control_chars = false,
            Arg::Zero => {
                self.eol = '\0';
                // TODO: Zero changes more than just this
            }
            Arg::GroupDirectoriesFirst => self.group_directories_first = true,
        }
        Ok(())
    }
}

#[test]
fn default() {
    assert_eq!(
        Settings::default().parse(["ls"]).unwrap().0,
        Settings {
            format: Format::Columns,
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
            width: 80,
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
    let (s, _operands) = Settings::default().parse(["ls", "--color"]).unwrap();
    assert!(s.color);

    let (s, _operands) = Settings::default().parse(["ls", "--color=always"]).unwrap();
    assert!(s.color);

    let (s, _operands) = Settings::default().parse(["ls", "--color=never"]).unwrap();
    assert!(!s.color);
}

#[test]
fn format() {
    let (s, _operands) = Settings::default().parse(["ls", "-l"]).unwrap();
    assert_eq!(s.format, Format::Long);

    let (s, _operands) = Settings::default().parse(["ls", "-m"]).unwrap();
    assert_eq!(s.format, Format::Commas);

    let (s, _operands) = Settings::default()
        .parse(["ls", "--format=across"])
        .unwrap();
    assert_eq!(s.format, Format::Across);

    let (s, _operands) = Settings::default().parse(["ls", "--format=acr"]).unwrap();
    assert_eq!(s.format, Format::Across);

    let (s, _operands) = Settings::default().parse(["ls", "-o"]).unwrap();
    assert_eq!(s.format, Format::Long);
    assert!(s.long_no_group && !s.long_no_owner && !s.long_numeric_uid_gid);

    let (s, _operands) = Settings::default().parse(["ls", "-g"]).unwrap();
    assert_eq!(s.format, Format::Long);
    assert!(!s.long_no_group && s.long_no_owner && !s.long_numeric_uid_gid);

    let (s, _operands) = Settings::default().parse(["ls", "-n"]).unwrap();
    assert_eq!(s.format, Format::Long);
    assert!(!s.long_no_group && !s.long_no_owner && s.long_numeric_uid_gid);

    let (s, _operands) = Settings::default().parse(["ls", "-og"]).unwrap();
    assert_eq!(s.format, Format::Long);
    assert!(s.long_no_group && s.long_no_owner && !s.long_numeric_uid_gid);

    let (s, _operands) = Settings::default().parse(["ls", "-on"]).unwrap();
    assert_eq!(s.format, Format::Long);
    assert!(s.long_no_group && !s.long_no_owner && s.long_numeric_uid_gid);

    let (s, _operands) = Settings::default().parse(["ls", "-onCl"]).unwrap();
    assert_eq!(s.format, Format::Long);
    assert!(s.long_no_group && !s.long_no_owner && s.long_numeric_uid_gid);
}

#[test]
fn time() {
    let (s, _operands) = Settings::default().parse(["ls", "--time=access"]).unwrap();
    assert_eq!(s.time, Time::Access);

    let (s, _operands) = Settings::default().parse(["ls", "--time=a"]).unwrap();
    assert_eq!(s.time, Time::Access);
}

#[test]
fn classify() {
    let (s, _operands) = Settings::default()
        .parse(["ls", "--indicator-style=classify"])
        .unwrap();
    assert_eq!(s.indicator_style, IndicatorStyle::Classify);

    let (s, _operands) = Settings::default().parse(["ls", "--classify"]).unwrap();
    assert_eq!(s.indicator_style, IndicatorStyle::Classify);

    let (s, _operands) = Settings::default()
        .parse(["ls", "--classify=always"])
        .unwrap();
    assert_eq!(s.indicator_style, IndicatorStyle::Classify);

    let (s, _operands) = Settings::default()
        .parse(["ls", "--classify=none"])
        .unwrap();
    assert_eq!(s.indicator_style, IndicatorStyle::None);

    let (s, _operands) = Settings::default().parse(["ls", "-F"]).unwrap();
    assert_eq!(s.indicator_style, IndicatorStyle::Classify);
}

#[test]
fn sort() {
    let (s, _operands) = Settings::default().parse(["ls", "--sort=time"]).unwrap();
    assert_eq!(s.sort, Sort::Time);

    let (s, _operands) = Settings::default().parse(["ls", "-X"]).unwrap();
    assert_eq!(s.sort, Sort::Extension);
}
