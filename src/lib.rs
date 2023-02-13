//! Argument parsing for the uutils coreutils project
//!
//! This crate provides the argument parsing for the
//! [uutils coreutils](https://www.github.com/uutils/coreutils)
//! It is designed to be flexible, while providing default
//! behaviour that aligns with GNU coreutils.
//!
//! # Features
//!
//!  - A derive macro for declarative argument definition.
//!  - Automatic help generation.
//!  - (Limited) markdown support in help text.
//!  - Positional and optional arguments.
//!  - Automatically parsing values into Rust types.
//!  - Define a custom exit code on errors.
//!  - Automatically accept unambiguous abbreviations of long options.
//!  - Handles invalid UTF-8 gracefully.
//!
//! # When you should not use this library
//!
//! The goal of this library is to make it easy to build applications that
//! mimic the behaviour of the GNU coreutils. There are other applications
//! that have similar behaviour, which are C application that use `getopt`
//! and `getopt_long`. If you want to mimic that behaviour exactly, this
//! is the library for you. If you want to write basically anything else,
//! you should probably pick another argument parser.
//!
//! # Getting Started
//!
//! Parsing with this library consists of two "phases". In the first
//! phase, the arguments are mapped to an iterator of an `enum`
//! implementing [`Arguments`]. The second phase is mapping these
//! arguments onto a `struct` implementing [`Options`]. By defining
//! your arguments this way, there is a clear divide between the public
//! API and the internal representation of the settings of your app.
//!
//! For more information on these traits, see their respective documentation:
//!
//! - [`Arguments`]
//! - [`Options`]
//!
//! Below is a minimal example of a full CLI application using this library.
//!
//! ```
//! use uutils_args::{Arguments, Initial, Options};
//!
//! #[derive(Arguments)]
//! enum Arg {
//!     // The doc strings below will be part of the `--help` text
//!     // First we define a simple flag:
//!     /// Do not transform input text to uppercase
//!     #[option("-n", "--no-caps")]
//!     NoCaps,
//!     
//!     // This option takes a value:    
//!     /// Add exclamation marks to output
//!     #[option("-e N", "--exclaim=N")]
//!     ExclamationMarks(u8),
//!
//!     // This is a positional argument, the range specifies that
//!     // at least one positional argument must be passed.
//!     #[positional(1..)]
//!     Text(String),
//! }
//!
//! #[derive(Initial)]
//! struct Settings {
//!     // We can change the default value with the field attribute.
//!     #[field(default = true)]
//!     caps: bool,
//!     exclamation_marks: u8,
//!     text: String,
//! }
//!
//! // To implement `Options`, we only need to provide the `apply` method.
//! // The `parse` method will be automatically generated.
//! impl Options for Settings {
//!     type Arg = Arg;
//!     fn apply(&mut self, arg: Arg) {
//!         match arg {
//!             Arg::NoCaps => self.caps = false,
//!             Arg::ExclamationMarks(n) => self.exclamation_marks += n,
//!             Arg::Text(s) => {
//!                 if self.text.is_empty() {
//!                     self.text.push_str(&s);
//!                 } else {
//!                     self.text.push(' ');
//!                     self.text.push_str(&s);
//!                 }
//!             }
//!         }
//!     }
//! }
//!
//! fn run(args: &'static [&'static str]) -> String {
//!     let s = Settings::parse(args);
//!     let mut output = if s.caps {
//!         s.text.to_uppercase()
//!     } else {
//!         s.text
//!     };
//!     for i in 0..s.exclamation_marks {
//!         output.push('!');
//!     }
//!     output
//! }
//!
//! // The first argument is the binary name. In this example it's ignored.
//! assert_eq!(run(&["shout", "hello"]), "HELLO");
//! assert_eq!(run(&["shout", "-e3", "hello"]), "HELLO!!!");
//! assert_eq!(run(&["shout", "-e", "3", "hello"]), "HELLO!!!");
//! assert_eq!(run(&["shout", "--no-caps", "hello"]), "hello");
//! assert_eq!(run(&["shout", "-e3", "-n", "hello"]), "hello!!!");
//! assert_eq!(run(&["shout", "-e3", "hello", "world"]), "HELLO WORLD!!!");
//! ```
//!
//! # Additional functionality
//!
//! To make it easier to implement [`Arguments`] and [`Options`], there are
//! two additional traits:
//!
//! - [`Initial`] is an alternative to the [`Default`] trait from the standard
//!   library, with a richer derive macro.
//! - [`FromValue`] allows for easy parsing from `OsStr` to any type
//!   implementing [`FromValue`]. This crate also provides a derive macro for
//!   this trait.
//!
//! # Examples
//!
//! The following files contain examples of commands defined with
//! `uutils_args`:
//!
//! - [hello world](https://github.com/tertsdiepraam/uutils-args/blob/main/examples/hello_world.rs)
//! - [arch](https://github.com/tertsdiepraam/uutils-args/blob/main/tests/coreutils/arch.rs)
//! - [b2sum](https://github.com/tertsdiepraam/uutils-args/blob/main/tests/coreutils/b2sum.rs)
//! - [base32](https://github.com/tertsdiepraam/uutils-args/blob/main/tests/coreutils/base32.rs)
//! - [basename](https://github.com/tertsdiepraam/uutils-args/blob/main/tests/coreutils/basename.rs)
//! - [cat](https://github.com/tertsdiepraam/uutils-args/blob/main/tests/coreutils/cat.rs)
//! - [echo](https://github.com/tertsdiepraam/uutils-args/blob/main/tests/coreutils/echo.rs)
//! - [ls](https://github.com/tertsdiepraam/uutils-args/blob/main/tests/coreutils/ls.rs)
//! - [mktemp](https://github.com/tertsdiepraam/uutils-args/blob/main/tests/coreutils/mktemp.rs)

mod error;
pub use derive::*;
pub use lexopt;
pub use term_md;

pub use error::Error;
use std::ffi::OsStr;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::{ffi::OsString, marker::PhantomData};

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
/// [derive macro](derive::Arguments) and does not need to be implemented
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

    /// Get the help string for this command.
    ///
    /// The `bin_name` specifies the name that executable was called with.
    fn help(bin_name: &str) -> String;

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
                    print!("{}", self.help());
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

    fn help(&self) -> String {
        T::help(self.parser.bin_name().unwrap())
    }

    fn version(&self) -> String {
        T::version()
    }
}

/// An alternative for the [`Default`](std::default::Default) trait, with a more feature
/// packed derive macro.
///
/// The `Initial` trait is used by `Options` to construct the initial
/// state of the options before any arguments are parsed.
///
/// The [derive macro](derive::Initial) supports setting the initial
/// value per field and parsing the initial values from environment
/// variables. Otherwise, it will be equivalent to the derive macro
/// for the [`Default`](std::default::Default) trait.
pub trait Initial: Sized {
    /// Create the initial state of `Self`
    fn initial() -> Self;
}

/// Defines the app settings by consuming [`Arguments`].
///
/// When implementing this trait, only two things need to be provided:
/// - the [`Arg`](Options::Arg) type, which defines the type to use for
///   argument parsing,
/// - the [`apply`](Options::apply) method, which defines to how map that
///   type onto the options.
///
/// By default, the [`Options::parse`] method will
/// 1. create a new instance of `Self` using [`Initial::initial`],
/// 2. repeatedly call [`ArgumentIter::next_arg`] and call [`Options::apply`]
///    on the result until the arguments are exhausted,
/// 3. and finally call [`Arguments::check_missing`].
pub trait Options: Sized + Initial {
    type Arg: Arguments;

    /// Apply a single argument to the options.
    fn apply(&mut self, arg: Self::Arg);

    /// Parse an iterator of arguments into
    fn parse<I>(args: I) -> Self
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        exit_if_err(Self::try_parse(args), Self::Arg::EXIT_CODE)
    }

    fn try_parse<I>(args: I) -> Result<Self, Error>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        let mut _self = Self::initial();
        let mut iter = Self::Arg::parse(args);
        while let Some(arg) = iter.next_arg()? {
            _self.apply(arg);
        }
        Self::Arg::check_missing(iter.positional_idx)?;
        Ok(_self)
    }
}

/// Defines how a type should be parsed from an argument.
pub trait FromValue: Sized {
    fn from_value(option: &str, value: OsString) -> Result<Self, Error>;
}

impl FromValue for OsString {
    fn from_value(_option: &str, value: OsString) -> Result<Self, Error> {
        Ok(value)
    }
}

impl FromValue for PathBuf {
    fn from_value(_option: &str, value: OsString) -> Result<Self, Error> {
        Ok(PathBuf::from(value))
    }
}

impl FromValue for String {
    fn from_value(_option: &str, value: OsString) -> Result<Self, Error> {
        match value.into_string() {
            Ok(s) => Ok(s),
            Err(os) => Err(Error::NonUnicodeValue(os)),
        }
    }
}

impl<T> FromValue for Option<T>
where
    T: FromValue,
{
    fn from_value(option: &str, value: OsString) -> Result<Self, Error> {
        Ok(Some(T::from_value(option, value)?))
    }
}

macro_rules! from_value_int {
    ($t: ty) => {
        impl FromValue for $t {
            fn from_value(option: &str, value: OsString) -> Result<Self, Error> {
                let value = String::from_value(option, value)?;
                value
                    .parse()
                    .map_err(|e: ParseIntError| Error::ParsingFailed {
                        value,
                        option: option.to_string(),
                        error: e.into(),
                    })
            }
        }
    };
}

from_value_int!(u8);
from_value_int!(u16);
from_value_int!(u32);
from_value_int!(u64);
from_value_int!(u128);
from_value_int!(usize);

from_value_int!(i8);
from_value_int!(i16);
from_value_int!(i32);
from_value_int!(i64);
from_value_int!(i128);
from_value_int!(isize);

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

pub fn parse_prefix<T: FromValue>(parser: &mut lexopt::Parser, prefix: &'static str) -> Option<T> {
    let mut raw = parser.try_raw_args()?;
    let arg = raw.peek()?.to_str()?;
    let value_str = arg.strip_prefix(prefix)?;

    // TODO: Give a nice flag name
    let value = T::from_value("", OsString::from(value_str)).ok()?;

    // Consume the argument we just parsed
    let _ = raw.next();

    Some(value)
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
