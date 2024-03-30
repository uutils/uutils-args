// spell-checker:ignore noxfer infile outfile iseek oseek conv iflag oflag iflags oflags
use std::path::PathBuf;

use uutils_args::{Arguments, Options, Value};

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
    Cbs(#[allow(unused)] usize),

    #[arg("skip=BYTES", "iseek=BYTES")]
    Skip(u64),

    #[arg("seek=BYTES", "oseek=BYTES")]
    Seek(u64),

    #[arg("count=N")]
    Count(usize),

    #[arg("status=LEVEL")]
    Status(StatusLevel),

    #[arg("conv=CONVERSIONS")]
    Conv(#[allow(unused)] String),

    #[arg("iflag=FLAGS")]
    Iflag(#[allow(unused)] String),

    #[arg("oflag=FLAGS")]
    Oflag(#[allow(unused)] String),
}

#[derive(Debug, PartialEq, Eq)]
struct Settings {
    infile: Option<PathBuf>,
    outfile: Option<PathBuf>,
    ibs: usize,
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

impl Default for Settings {
    fn default() -> Self {
        Self {
            ibs: 512,
            obs: 512,
            infile: Default::default(),
            outfile: Default::default(),
            skip: Default::default(),
            seek: Default::default(),
            count: Default::default(),
            _iconv: Default::default(),
            _iflags: Default::default(),
            _oconv: Default::default(),
            _oflags: Default::default(),
            status: Default::default(),
        }
    }
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
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
        Ok(())
    }
}

#[test]
fn empty() {
    assert_eq!(
        Settings::default().parse(["dd"]).unwrap().0,
        Settings::default()
    )
}

#[test]
fn infile() {
    assert_eq!(
        Settings::default().parse(["dd", "if=hello"]).unwrap().0,
        Settings {
            infile: Some(PathBuf::from("hello")),
            ..Settings::default()
        }
    )
}

#[test]
fn outfile() {
    assert_eq!(
        Settings::default().parse(["dd", "of=hello"]).unwrap().0,
        Settings {
            outfile: Some(PathBuf::from("hello")),
            ..Settings::default()
        }
    )
}

#[test]
fn bs() {
    assert_eq!(
        Settings::default().parse(["dd", "ibs=1"]).unwrap().0,
        Settings {
            ibs: 1,
            obs: 512,
            ..Settings::default()
        }
    );
    assert_eq!(
        Settings::default().parse(["dd", "obs=1"]).unwrap().0,
        Settings {
            ibs: 512,
            obs: 1,
            ..Settings::default()
        }
    );
    assert_eq!(
        Settings::default()
            .parse(["dd", "ibs=10", "obs=1"])
            .unwrap()
            .0,
        Settings {
            ibs: 10,
            obs: 1,
            ..Settings::default()
        }
    );
    assert_eq!(
        Settings::default()
            .parse(["dd", "ibs=10", "bs=1"])
            .unwrap()
            .0,
        Settings {
            ibs: 1,
            obs: 1,
            ..Settings::default()
        }
    )
}
