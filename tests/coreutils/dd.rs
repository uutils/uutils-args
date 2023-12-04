// spell-checker:ignore noxfer infile outfile iseek oseek conv iflag oflag iflags oflags
use std::path::PathBuf;

use uutils_args::{Arguments, Initial, Options, Value};

#[derive(Value, Debug, PartialEq, Eq)]
enum StatusLevel {
    #[value("none")]
    None,
    #[value("noxfer")]
    Noxfer,
    #[value("progress")]
    Progress,
}

// TODO: The bytes arguments should parse sizes
#[derive(Arguments)]
enum Arg {
    #[arg("if=FILE")]
    Infile(PathBuf),

    #[arg("of=FILE")]
    Outfile(PathBuf),

    #[arg("ibs=BYTES")]
    Ibs(usize),

    #[arg("obs=BYTES")]
    Obs(usize),

    #[arg("bs=BYTES")]
    Bs(usize),

    #[arg("cbs=BYTES")]
    Cbs(usize),

    #[arg("skip=BYTES", "iseek=BYTES")]
    Skip(u64),

    #[arg("seek=BYTES", "oseek=BYTES")]
    Seek(u64),

    #[arg("count=N")]
    Count(usize),

    #[arg("status=LEVEL")]
    Status(StatusLevel),

    #[arg("conv=CONVERSIONS")]
    Conv(String),

    #[arg("iflag=FLAGS")]
    Iflag(String),

    #[arg("oflag=FLAGS")]
    Oflag(String),
}

#[derive(Initial, Debug, PartialEq, Eq)]
struct Settings {
    infile: Option<PathBuf>,
    outfile: Option<PathBuf>,
    #[initial(512)]
    ibs: usize,
    #[initial(512)]
    obs: usize,
    skip: u64,
    seek: u64,
    count: usize,
    _iconv: Vec<String>,
    _iflags: Vec<String>,
    _oconv: Vec<String>,
    _oflags: Vec<String>,
    status: Option<StatusLevel>,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::Infile(f) => self.infile = Some(f),
            Arg::Outfile(f) => self.outfile = Some(f),
            Arg::Ibs(b) => self.ibs = b,
            Arg::Obs(b) => self.obs = b,
            Arg::Bs(b) => {
                self.ibs = b;
                self.obs = b;
            }
            Arg::Cbs(_) => todo!(),
            Arg::Skip(b) => self.skip = b,
            Arg::Seek(b) => self.seek = b,
            Arg::Count(n) => self.count = n,
            Arg::Status(level) => self.status = Some(level),
            Arg::Conv(_) => todo!(),
            Arg::Iflag(_) => todo!(),
            Arg::Oflag(_) => todo!(),
        }
    }
}

#[test]
fn empty() {
    assert_eq!(Settings::try_parse(["dd"]).unwrap(), Settings::initial())
}

#[test]
fn infile() {
    assert_eq!(
        Settings::try_parse(["dd", "if=hello"]).unwrap(),
        Settings {
            infile: Some(PathBuf::from("hello")),
            ..Settings::initial()
        }
    )
}

#[test]
fn outfile() {
    assert_eq!(
        Settings::try_parse(["dd", "of=hello"]).unwrap(),
        Settings {
            outfile: Some(PathBuf::from("hello")),
            ..Settings::initial()
        }
    )
}

#[test]
fn bs() {
    assert_eq!(
        Settings::try_parse(["dd", "ibs=1"]).unwrap(),
        Settings {
            ibs: 1,
            obs: 512,
            ..Settings::initial()
        }
    );
    assert_eq!(
        Settings::try_parse(["dd", "obs=1"]).unwrap(),
        Settings {
            ibs: 512,
            obs: 1,
            ..Settings::initial()
        }
    );
    assert_eq!(
        Settings::try_parse(["dd", "ibs=10", "obs=1"]).unwrap(),
        Settings {
            ibs: 10,
            obs: 1,
            ..Settings::initial()
        }
    );
    assert_eq!(
        Settings::try_parse(["dd", "ibs=10", "bs=1"]).unwrap(),
        Settings {
            ibs: 1,
            obs: 1,
            ..Settings::initial()
        }
    )
}
