// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

#![doc = include_str!("../README.md")]

mod error;
mod value;

pub use lexopt;
pub use uutils_args_derive::*;

pub use error::Error;
pub use value::{Value, ValueError, ValueResult};

use std::{
    ffi::{OsStr, OsString},
    marker::PhantomData,
};

/// A wrapper around a type implementing [`Arguments`] that adds `Help`
/// and `Version` variants.
#[derive(Clone)]
pub enum Argument<T: Arguments> {
    Help,
    Version,
    Custom(T),
}

fn exit_if_err<T>(res: Result<T, Error>, exit_code: i32) -> T {
    match res {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(exit_code);
        }
    }
}

/// Defines how the arguments are parsed.
///
/// If a type `T` implements this trait, we can construct an `ArgumentIter<T>`,
/// meaning that we can parse the individual arguments to `T`.\
///
/// Usually, this trait will be implemented via the
/// [derive macro](derive@Arguments) and does not need to be implemented
/// manually.
pub trait Arguments: Sized {
    /// The exit code to exit the program with on error.
    const EXIT_CODE: i32;

    /// Parse an iterator of arguments into an
    /// [`ArgumentIter<Self>`](ArgumentIter).
    fn parse<I>(args: I) -> ArgumentIter<Self>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        ArgumentIter::<Self>::from_args(args)
    }

    /// Parse the next argument from the lexopt parser.
    ///
    /// This method is called by [`ArgumentIter::next_arg`].
    fn next_arg(
        parser: &mut lexopt::Parser,
        positional_idx: &mut usize,
    ) -> Result<Option<Argument<Self>>, Error>;

    /// Check for any required arguments that have not been found.
    ///
    /// If any missing arguments are found, the appropriate error is returned.
    /// The `positional_idx` parameter specifies how many positional arguments
    /// have been passed so far. This method is called at the end of
    /// [`Options::parse`] and [`Options::try_parse`].
    fn check_missing(positional_idx: usize) -> Result<(), Error>;

    /// Print the help string for this command.
    ///
    /// The `bin_name` specifies the name that executable was called with.
    fn help(bin_name: &str) -> std::io::Result<()>;

    /// Get the version string for this command.
    fn version() -> String;

    /// Check all arguments immediately and exit on errors.
    ///
    /// This is useful if you want to validate the arguments. This method will
    /// exit if `--help` or `--version` are passed and if any errors are found.
    fn check<I>(args: I)
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        exit_if_err(Self::try_check(args), Self::EXIT_CODE)
    }

    /// Check all arguments immediately and return any errors.
    ///
    /// This is useful if you want to validate the arguments. This method will
    /// exit if `--help` or `--version` are passed.
    fn try_check<I>(args: I) -> Result<(), Error>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        let mut iter = Self::parse(args);
        while iter.next_arg()?.is_some() {}
        Ok(())
    }
}

/// An iterator over arguments.
///
/// Can be constructed by calling [`Arguments::parse`]. Usually, this method
/// won't be used directly, but is used internally in [`Options::parse`] and
/// [`Options::try_parse`].
pub struct ArgumentIter<T: Arguments> {
    parser: lexopt::Parser,
    pub positional_idx: usize,
    t: PhantomData<T>,
}

impl<T: Arguments> ArgumentIter<T> {
    fn from_args<I>(args: I) -> Self
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        Self {
            parser: lexopt::Parser::from_iter(args),
            positional_idx: 0,
            t: PhantomData,
        }
    }

    pub fn next_arg(&mut self) -> Result<Option<T>, Error> {
        if let Some(arg) = T::next_arg(&mut self.parser, &mut self.positional_idx)? {
            match arg {
                Argument::Help => {
                    self.help()?;
                    std::process::exit(0);
                }
                Argument::Version => {
                    print!("{}", self.version());
                    std::process::exit(0);
                }
                Argument::Custom(arg) => Ok(Some(arg)),
            }
        } else {
            Ok(None)
        }
    }

    fn help(&self) -> std::io::Result<()> {
        T::help(self.parser.bin_name().unwrap())
    }

    fn version(&self) -> String {
        T::version()
    }
}

/// An alternative for the [`Default`] trait, with a more feature
/// packed derive macro.
///
/// The `Initial` trait is used by `Options` to construct the initial
/// state of the options before any arguments are parsed.
///
/// The [derive macro](derive@Initial) supports setting the initial
/// value per field and parsing the initial values from environment
/// variables. Otherwise, it will be equivalent to the derive macro
/// for the [`Default`] trait.
pub trait Initial: Sized {
    /// Create the initial state of `Self`
    fn initial() -> Self;
}

/// Defines the app settings by consuming [`Arguments`].
///
/// When implementing this trait, only two things need to be provided:
/// - the `Arg` type parameter, which defines the type to use for
///   argument parsing,
/// - the [`apply`](Options::apply) method, which defines to how map that
///   type onto the options.
///
/// By default, the [`Options::parse`] method will
/// 1. create a new instance of `Self` using [`Initial::initial`],
/// 2. repeatedly call [`ArgumentIter::next_arg`] and call [`Options::apply`]
///    on the result until the arguments are exhausted,
/// 3. and finally call [`Arguments::check_missing`].
pub trait Options<Arg: Arguments>: Sized + Initial {
    /// Apply a single argument to the options.
    fn apply(&mut self, arg: Arg);

    /// Parse an iterator of arguments into
    fn parse<I>(args: I) -> Self
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        exit_if_err(Self::try_parse(args), Arg::EXIT_CODE)
    }

    fn try_parse<I>(args: I) -> Result<Self, Error>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        let mut _self = Self::initial();
        let mut iter = Arg::parse(args);
        while let Some(arg) = iter.next_arg()? {
            _self.apply(arg);
        }
        Arg::check_missing(iter.positional_idx)?;
        Ok(_self)
    }
}

/// Parses an echo-style positional argument
///
/// This means that any argument that does not solely consist of a hyphen
/// followed by the characters in the list of `short_args` is considered
/// to be a positional argument, instead of an invalid argument. This
/// includes the `--` argument, which is ignored by `echo`.
///
/// This function is hidden and prefixed with `__` because it should only
/// be called via the derive macros.
#[doc(hidden)]
pub fn __echo_style_positional(p: &mut lexopt::Parser, short_args: &[char]) -> Option<OsString> {
    let mut raw = p.try_raw_args()?;
    let val = raw.peek()?;

    if is_echo_style_positional(val, short_args) {
        let val = val.into();
        raw.next();
        Some(val)
    } else {
        None
    }
}

fn is_echo_style_positional(s: &OsStr, short_args: &[char]) -> bool {
    let s = match s.to_str() {
        Some(x) => x,
        // If it's invalid utf-8 then it can't be a short arg, so must
        // be a positional argument.
        None => return true,
    };
    let mut chars = s.chars();
    let is_short_args = chars.next() == Some('-') && chars.all(|c| short_args.contains(&c));
    !is_short_args
}

/// Parse an argument defined by a prefix
#[doc(hidden)]
pub fn parse_prefix<T: Value>(parser: &mut lexopt::Parser, prefix: &'static str) -> Option<T> {
    let mut raw = parser.try_raw_args()?;

    // TODO: The to_str call is a limitation. Maybe we need to pull in something like bstr
    let arg = raw.peek()?.to_str()?;
    let value_str = arg.strip_prefix(prefix)?;

    let value = T::from_value(OsStr::new(value_str)).ok()?;

    // Consume the argument we just parsed
    let _ = raw.next();

    Some(value)
}

/// Parse a value and wrap the error into an `Error::ParsingFailed`
#[doc(hidden)]
pub fn parse_value_for_option<T: Value>(opt: &str, v: &OsStr) -> Result<T, Error> {
    T::from_value(v).map_err(|e| Error::ParsingFailed {
        option: opt.into(),
        value: v.to_string_lossy().to_string(),
        error: e,
    })
}

pub fn infer_long_option<'a>(
    input: &'a str,
    long_options: &'a [&'a str],
) -> Result<&'a str, Error> {
    let mut candidates = Vec::new();
    let mut exact_match = None;
    for opt in long_options {
        if *opt == input {
            exact_match = Some(opt);
            break;
        } else if opt.starts_with(input) {
            candidates.push(opt);
        }
    }

    match (exact_match, &candidates[..]) {
        (Some(opt), _) => Ok(*opt),
        (None, [opt]) => Ok(**opt),
        (None, []) => Err(Error::UnexpectedOption(
            format!("--{input}"),
            filter_suggestions(input, long_options, "--"),
        )),
        (None, _) => Err(Error::AmbiguousOption {
            option: input.to_string(),
            candidates: candidates.iter().map(|s| s.to_string()).collect(),
        }),
    }
}

/// Filter a list of options to just the elements that are similar to the given string
pub fn filter_suggestions(input: &str, long_options: &[&str], prefix: &str) -> Vec<String> {
    long_options
        .iter()
        .filter(|opt| strsim::jaro(input, opt) > 0.7)
        .map(|o| format!("{prefix}{o}"))
        .collect()
}

#[cfg(test)]
mod test {
    use std::ffi::OsStr;

    use crate::is_echo_style_positional;

    #[test]
    fn echo_positional() {
        assert!(is_echo_style_positional(OsStr::new("-aaa"), &['b']));
        assert!(is_echo_style_positional(OsStr::new("--"), &['b']));
        assert!(!is_echo_style_positional(OsStr::new("-b"), &['b']));
    }
}
