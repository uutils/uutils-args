// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

#![doc = include_str!("../README.md")]

mod error;
pub mod internal;
mod value;

pub use lexopt;
pub use uutils_args_derive::*;

pub use error::{Error, ErrorKind};
pub use value::{Value, ValueError, ValueResult};

use std::{ffi::OsString, marker::PhantomData};

/// A wrapper around a type implementing [`Arguments`] that adds `Help`
/// and `Version` variants.
#[derive(Clone)]
pub enum Argument<T: Arguments> {
    Help,
    Version,
    Custom(T),
}

fn exit_if_err<T>(res: Result<T, Error>) -> T {
    match res {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(err.exit_code);
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
    ) -> Result<Option<Argument<Self>>, ErrorKind>;

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
        exit_if_err(Self::try_check(args))
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

    #[cfg(feature = "complete")]
    fn complete() -> uutils_args_complete::Command<'static>;
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
        if let Some(arg) =
            T::next_arg(&mut self.parser, &mut self.positional_idx).map_err(|kind| Error {
                exit_code: T::EXIT_CODE,
                kind,
            })?
        {
            match arg {
                Argument::Help => {
                    self.help().unwrap();
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

/// Defines the app settings by consuming [`Arguments`].
///
/// When implementing this trait, only two things need to be provided:
/// - the `Arg` type parameter, which defines the type to use for
///   argument parsing,
/// - the [`apply`](Options::apply) method, which defines to how map that
///   type onto the options.
///
/// By default, the [`Options::parse`] method will
/// 1. repeatedly call [`ArgumentIter::next_arg`] and call [`Options::apply`]
///    on the result until the arguments are exhausted,
/// 2. and finally call [`Arguments::check_missing`] to check whether all
///    required arguments were given.
pub trait Options<Arg: Arguments>: Sized {
    /// Apply a single argument to the options.
    fn apply(&mut self, arg: Arg);

    /// Parse an iterator of arguments into the options
    fn parse<I>(self, args: I) -> Self
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        exit_if_err(self.try_parse(args))
    }

    #[allow(unused_mut)]
    fn try_parse<I>(mut self, args: I) -> Result<Self, Error>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        // Hacky but it works: if the parse-is-complete flag is active the
        // parse function becomes the complete function so that no additional
        // functionality is necessary for users to generate completions. It is
        // important that we exit the program here, because the program does
        // not expect us to print the completion here and therefore will behave
        // incorrectly.
        #[cfg(feature = "parse-is-complete")]
        {
            print_complete::<_, Self, Arg>(args.into_iter());
            std::process::exit(0);
        }

        #[cfg(not(feature = "parse-is-complete"))]
        {
            let mut iter = Arg::parse(args);
            while let Some(arg) = iter.next_arg()? {
                self.apply(arg);
            }
            Arg::check_missing(iter.positional_idx)?;
            Ok(self)
        }
    }

    #[cfg(feature = "complete")]
    fn complete(shell: &str) -> String {
        uutils_args_complete::render(&Arg::complete(), shell)
    }
}

#[cfg(feature = "parse-is-complete")]
fn print_complete<I, O: Options<Arg>, Arg: Arguments>(mut args: I)
where
    I: Iterator + 'static,
    I::Item: Into<OsString>,
{
    let _exec_name = args.next();
    let shell = args
        .next()
        .expect("Need a shell argument for completion.")
        .into();
    let shell = shell.to_string_lossy();
    assert!(args.next().is_none(), "completion only takes one argument");
    println!("{}", O::complete(&shell));
}
