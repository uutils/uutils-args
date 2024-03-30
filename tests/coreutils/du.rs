use std::ffi::OsString;
use uutils_args::{Arguments, Options};

#[derive(Arguments)]
enum Arg {
    #[arg("--apparent-size")]
    ApparentSize,

    #[arg("-B[SIZE]")]
    #[arg("--block-size[=SIZE]")]
    BlockSize(OsString),

    #[arg("-b")]
    #[arg("--bytes")]
    Bytes,

    #[arg("-k")]
    KibiBytes,

    #[arg("-m")]
    MibiBytes,
    // Note that --si and -h only affect the *output formatting*,
    // and not the size determination itself.
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Settings {
    apparent_size: bool,
    block_size_str: Option<OsString>,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::ApparentSize => self.apparent_size = true,
            Arg::BlockSize(os_str) => self.block_size_str = Some(os_str),
            Arg::Bytes => {
                self.apparent_size = true;
                self.block_size_str = Some("1".into());
            }
            Arg::KibiBytes => self.block_size_str = Some("K".into()),
            Arg::MibiBytes => self.block_size_str = Some("M".into()),
        }
        Ok(())
    }
}

#[test]
fn noarg() {
    let (settings, operands) = Settings::default().parse(["date"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings,
        Settings {
            apparent_size: false,
            block_size_str: None,
        }
    );
}

#[test]
fn bytes() {
    let (settings, operands) = Settings::default().parse(["date", "-b"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings,
        Settings {
            apparent_size: true,
            block_size_str: Some("1".into()),
        }
    );
}

#[test]
fn kibibytes() {
    let (settings, operands) = Settings::default().parse(["date", "-k"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings,
        Settings {
            apparent_size: false,
            block_size_str: Some("K".into()),
        }
    );
}

#[test]
fn bytes_kibibytes() {
    let (settings, operands) = Settings::default().parse(["date", "-bk"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings,
        Settings {
            apparent_size: true,
            block_size_str: Some("K".into()),
        }
    );
}

#[test]
fn kibibytes_bytes() {
    let (settings, operands) = Settings::default().parse(["date", "-kb"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings,
        Settings {
            apparent_size: true,
            block_size_str: Some("1".into()),
        }
    );
}

#[test]
fn apparent_size() {
    let (settings, operands) = Settings::default()
        .parse(["date", "--apparent-size"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings,
        Settings {
            apparent_size: true,
            block_size_str: None,
        }
    );
}

#[test]
fn mibibytes() {
    let (settings, operands) = Settings::default().parse(["date", "-m"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings,
        Settings {
            apparent_size: false,
            block_size_str: Some("M".into()),
        }
    );
}

#[test]
fn all() {
    let (settings, operands) = Settings::default()
        .parse(["date", "--apparent-size", "-bkm", "-B123"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings,
        Settings {
            apparent_size: true,
            block_size_str: Some("123".into()),
        }
    );
}
