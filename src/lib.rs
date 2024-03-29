// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! <div style="font-size: 2em; font-family: 'Fira Sans',Arial,NanumBarunGothic,sans-serif; border: 1px solid var(--link-color); border-radius: 4px; padding: 10px;">
//!
//! [Click here for the guide](docs::guide)
//!
//! </div>
//!
#![doc = include_str!("../README.md")]

mod error;
pub mod internal;
pub mod positional;
mod value;

#[cfg(doc)]
pub mod docs;

pub use lexopt;

// The documentation for the derive macros is written here instead of in
// `uutils_args_derive`, because we need to be able to link to items and the
// documentation in this crate.

/// Derive macro for [`Value`](trait@crate::Value)
///
/// [See also the chapter on this trait in the guide](crate::docs::guide::value)
///
/// This macro only works on `enums` and will error at compile time when it is
/// used on a `struct`.
pub use uutils_args_derive::Value;

/// Derive macro for [`Arguments`](trait@crate::Arguments)
///
/// [See also the chapter on this trait in the guide](crate::docs::guide::quick)
///
/// This macro only works on `enums` and will error at compile time when it is
/// used on a `struct`.
///
/// /// ## Argument specifications
///
/// | specification  | kind       | value    |
/// | -------------- | ---------- | -------- |
/// | `VAL`          | positional | n.a.     |
/// | `-s`           | short      | none     |
/// | `-s VAL`       | short      | required |
/// | `-s[VAL]`      | short      | optional |
/// | `--long`       | long       | none     |
/// | `--long=VAL`   | long       | required |
/// | `--long[=VAL]` | long       | optional |
/// | `long=VAL`     | dd         | required |
///
pub use uutils_args_derive::Arguments;

pub use error::{Error, ErrorKind};
pub use value::{Value, ValueError, ValueResult};

use std::{ffi::OsString, marker::PhantomData};

/// A wrapper around a type implementing [`Arguments`] that adds `Help`
/// and `Version` variants.
#[derive(Clone)]
pub enum Argument<T: Arguments> {
    Help,
    Version,
    Positional(OsString),
    MultiPositional(Vec<OsString>),
    Custom(T),
}

/// Defines how the arguments are parsed.
///
/// Usually, this trait will be implemented via the
/// [derive macro](derive@Arguments) and does not need to be implemented
/// manually.
pub trait Arguments: Sized {
    /// The exit code to exit the program with on error.
    const EXIT_CODE: i32;

    /// Parse the next argument from the lexopt parser.
    fn next_arg(parser: &mut lexopt::Parser) -> Result<Option<Argument<Self>>, ErrorKind>;

    /// Print the help string for this command.
    ///
    /// The `bin_name` specifies the name that executable was called with.
    fn help(bin_name: &str) -> String;

    /// Get the version string for this command.
    fn version() -> String;

    /// Check all arguments immediately and return any errors.
    ///
    /// This is useful if you want to validate the arguments. This method will
    /// exit if `--help` or `--version` are passed.
    fn check<I>(args: I) -> Result<(), Error>
    where
        I: IntoIterator,
        I::Item: Into<OsString>,
    {
        let mut iter = ArgumentIter::<Self>::from_args(args);
        while iter.next_arg()?.is_some() {}
        Ok(())
    }

    #[cfg(feature = "complete")]
    fn complete() -> uutils_args_complete::Command<'static>;
}

/// An iterator over arguments.
struct ArgumentIter<T: Arguments> {
    parser: lexopt::Parser,
    positional_arguments: Vec<OsString>,
    t: PhantomData<T>,
}

impl<T: Arguments> ArgumentIter<T> {
    fn from_args<I>(args: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<OsString>,
    {
        Self {
            parser: lexopt::Parser::from_iter(args),
            positional_arguments: Vec::new(),
            t: PhantomData,
        }
    }

    pub fn next_arg(&mut self) -> Result<Option<T>, Error> {
        while let Some(arg) = T::next_arg(&mut self.parser).map_err(|kind| Error {
            exit_code: T::EXIT_CODE,
            kind,
        })? {
            match arg {
                Argument::Help => {
                    print!("{}", T::help(self.parser.bin_name().unwrap()));
                    std::process::exit(0);
                }
                Argument::Version => {
                    print!("{}", T::version());
                    std::process::exit(0);
                }
                Argument::Positional(arg) => {
                    self.positional_arguments.push(arg);
                }
                Argument::MultiPositional(args) => {
                    self.positional_arguments.extend(args);
                }
                Argument::Custom(arg) => return Ok(Some(arg)),
            }
        }
        Ok(None)
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
/// By default, the [`Options::parse`] method iterate over the arguments and
/// call [`Options::apply`] on the result until the arguments are exhausted.
pub trait Options<Arg: Arguments>: Sized {
    /// Apply a single argument to the options.
    fn apply(&mut self, arg: Arg) -> Result<(), Error>;

    /// Parse an iterator of arguments into the options
    #[allow(unused_mut)]
    fn parse<I>(mut self, args: I) -> Result<(Self, Vec<OsString>), Error>
    where
        I: IntoIterator,
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
            let mut iter = ArgumentIter::<Arg>::from_args(args);
            while let Some(arg) = iter.next_arg()? {
                self.apply(arg)?;
            }
            Ok((self, iter.positional_arguments))
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
    I: Iterator,
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
