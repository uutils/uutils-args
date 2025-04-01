use std::{ffi::OsString, path::PathBuf};

use uutils_args::{Arguments, Options, Value};

// This format is way to specific to implement using a library. Basically, any
// deviation should be return `None` to indicate that we're not using the
// this format. If this fails, we fall back on the normal parsing, so errors
// from this function are not relevant, so we can just return an `Option`.
// Once this gets into uutils, I highly recommend that we make this format
// optional at compile time. As the GNU docs explain, it's very error-prone.
fn parse_deprecated<I>(iter: I) -> Option<(Settings, Vec<OsString>)>
where
    I: IntoIterator + Clone,
    I::Item: Into<OsString>,
{
    let mut iter = iter.into_iter();

    // We don't use it, but the first argument is the binary name.
    iter.next()?;

    let shorthand = iter.next()?;
    let input = iter.next()?;

    // We can only have a maximum of 2 arguments in this format
    // The error doesn't really matter because we'll ignore any errors
    // from this format.
    if iter.next().is_some() {
        return None;
    }

    // Parse the shorthand by turning it into a String (via OsString)
    // The format we're parsing is `-[NUM][bkm][cqv]`.
    let os_string = shorthand.into();

    // The part of the string that is not parsed yet
    let mut rest = os_string.to_str()?;

    // Corner case: If it's just `-` then it needs to be parsed like
    // the non-deprecated syntax, because `-` represents standard input.
    if rest == "-" {
        return None;
    }

    // Parse the sign, only '-' is supported.
    rest = rest.strip_prefix('-')?;

    // Find and parse the number part of the string
    let end_num = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());

    // In `head`, the shorthand must start with a number.
    // `-k` (which fails) and `-c`, etc. are parsed as normal.
    if end_num == 0 {
        return None;
    }

    let num = rest[..end_num].parse().unwrap_or(10);
    rest = &rest[end_num..];

    // Parse the other options (`-c`, `-q`, `-v`), which can appear any number
    // of times. `-z` is also supported, though that is undocumented in GNU.
    let mut mode = Mode::Lines;
    let mut verbose = false;
    let mut zero = false;
    for char in rest.chars() {
        match char {
            'c' => mode = Mode::Bytes,
            'q' => verbose = false,
            'v' => verbose = true,
            'z' => zero = true,
            _ => return None,
        }
    }

    Some((
        Settings {
            number: SigNum::Negative(num),
            mode,
            verbose,
            zero,
        },
        vec![input.into()],
    ))
}

#[derive(Arguments)]
enum Arg {
    #[arg("-c NUM", "--bytes=NUM")]
    Bytes(SigNum),

    #[arg("-n NUM", "--lines=NUM")]
    Lines(SigNum),

    #[arg("-q", "--quiet", "--silent")]
    Quiet,

    #[arg("-v", "--verbose")]
    Verbose,

    #[arg("-z", "--zero-terminated")]
    Zero,
}

// We need both negative and positive 0
#[derive(Debug, PartialEq, Eq)]
enum SigNum {
    Positive(u64),
    Negative(u64),
}

impl Value for SigNum {
    fn from_value(value: &std::ffi::OsStr) -> uutils_args::ValueResult<Self> {
        let s = String::from_value(value)?;
        let mut rest: &str = &s;

        let sign = if let Some(r) = s.strip_prefix('+') {
            rest = r;
            Self::Positive
        } else if let Some(r) = s.strip_prefix('-') {
            rest = r;
            Self::Negative
        } else {
            Self::Negative
        };

        // Get the number from the front of the string
        let end_num = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        let num = rest[..end_num].parse().unwrap_or(1);
        rest = &rest[end_num..];

        // Determine the multiplier
        // We're being a bit overly defensive here. I'm assuming it will
        // be optimized away.
        let multiplier: Option<u64> = match rest {
            "" => Some(1),
            "b" => Some(512),
            "K" | "KiB" => Some(1024),
            "M" | "MiB" => 1024_u64.checked_pow(2),
            "G" | "GiB" => 1024_u64.checked_pow(3),
            "T" | "TiB" => 1024_u64.checked_pow(4),
            "P" | "PiB" => 1024_u64.checked_pow(5),
            "E" | "EiB" => 1024_u64.checked_pow(6),
            "Z" | "ZiB" => 1024_u64.checked_pow(7),
            "Y" | "YiB" => 1024_u64.checked_pow(8),
            "KB" => Some(1000),
            "MB" => 1000_u64.checked_pow(2),
            "GB" => 1000_u64.checked_pow(3),
            "TB" => 1000_u64.checked_pow(4),
            "PB" => 1000_u64.checked_pow(5),
            "EB" => 1000_u64.checked_pow(6),
            "ZB" => 1000_u64.checked_pow(7),
            "YB" => 1000_u64.checked_pow(8),
            _ => return Err(format!("Invalid number of lines: {s}").into()),
        };

        let number = match multiplier.and_then(|m| m.checked_mul(num)) {
            Some(number) => number,
            None => return Err("Value too large for defined data type".into()),
        };

        Ok(sign(number))
    }
}

impl Default for SigNum {
    fn default() -> Self {
        Self::Negative(10)
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Mode {
    Bytes,
    #[default]
    Lines,
}

#[derive(Default)]
struct Settings {
    mode: Mode,
    number: SigNum,
    // TODO: Should be a dedicated PID type
    verbose: bool,
    zero: bool,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::Bytes(n) => {
                self.mode = Mode::Bytes;
                self.number = n;
            }
            Arg::Lines(n) => {
                self.mode = Mode::Lines;
                self.number = n;
            }
            Arg::Quiet => self.verbose = false,
            Arg::Verbose => self.verbose = true,
            Arg::Zero => self.zero = true,
        }
        Ok(())
    }
}

fn parse_head<I>(iter: I) -> Result<(Settings, Vec<OsString>), uutils_args::Error>
where
    I: IntoIterator + Clone,
    I::Item: Into<OsString>,
{
    match parse_deprecated(iter.clone()) {
        Some(s) => Ok(s),
        None => Settings::default().parse(iter),
    }
}

#[test]
fn shorthand() {
    let (s, _operands) = parse_head(["head", "-20", "some_file"]).unwrap();
    assert_eq!(s.number, SigNum::Negative(20));
    assert_eq!(s.mode, Mode::Lines);

    let (s, _operands) = parse_head(["head", "-100cq", "some_file"]).unwrap();
    assert_eq!(s.number, SigNum::Negative(100));
    assert_eq!(s.mode, Mode::Bytes);

    // Corner case where the shorthand does not apply
    let (s, operands) = parse_head(["head", "-c", "42"]).unwrap();
    assert_eq!(s.number, SigNum::Negative(42));
    assert_eq!(s.mode, Mode::Bytes);
    assert_eq!(operands, Vec::<PathBuf>::new());
}

#[test]
fn standard_input() {
    let (_s, operands) = parse_head(["head", "-"]).unwrap();
    assert_eq!(operands, vec![PathBuf::from("-")])
}

#[test]
fn normal_format() {
    let (s, _operands) = parse_head(["head", "-c", "20", "some_file"]).unwrap();
    assert_eq!(s.number, SigNum::Negative(20));
    assert_eq!(s.mode, Mode::Bytes);
}

#[test]
fn signum() {
    let (s, _operands) = parse_head(["head", "-n", "20"]).unwrap();
    assert_eq!(s.number, SigNum::Negative(20));
    let (s, _operands) = parse_head(["head", "-n", "-20"]).unwrap();
    assert_eq!(s.number, SigNum::Negative(20));
    let (s, _operands) = parse_head(["head", "-n", "+20"]).unwrap();
    assert_eq!(s.number, SigNum::Positive(20));

    let (s, _operands) = parse_head(["head", "-n", "20b"]).unwrap();
    assert_eq!(s.number, SigNum::Negative(20 * 512));
    let (s, _operands) = parse_head(["head", "-n", "+20b"]).unwrap();
    assert_eq!(s.number, SigNum::Positive(20 * 512));

    let (s, _operands) = parse_head(["head", "-n", "b"]).unwrap();
    assert_eq!(s.number, SigNum::Negative(512));
    let (s, _operands) = parse_head(["head", "-n", "+b"]).unwrap();
    assert_eq!(s.number, SigNum::Positive(512));

    assert!(parse_head(["head", "-n", "20invalid_suffix"]).is_err());
}
