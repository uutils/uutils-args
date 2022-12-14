pub use derive::*;
pub use lexopt;
pub use term_md;

use std::error::Error as StdError;
use std::num::ParseIntError;
use std::{ffi::OsString, marker::PhantomData};

#[derive(Debug)]
pub enum Error {
    MissingValue {
        option: Option<String>,
    },
    MissingPositionalArguments(Vec<String>),
    UnexpectedOption(String),
    UnexpectedArgument(OsString),
    UnexpectedValue {
        option: String,
        value: OsString,
    },
    ParsingFailed {
        value: String,
        error: Box<dyn StdError + Send + Sync + 'static>,
    },
    AmbiguousOption {
        option: String,
        candidates: Vec<String>,
    },
    NonUnicodeValue(OsString),
    Custom(Box<dyn StdError + Send + Sync + 'static>),
}

impl From<lexopt::Error> for Error {
    fn from(other: lexopt::Error) -> Error {
        match other {
            lexopt::Error::MissingValue { option } => Self::MissingValue { option },
            lexopt::Error::UnexpectedOption(s) => Self::UnexpectedOption(s),
            lexopt::Error::UnexpectedArgument(s) => Self::UnexpectedArgument(s),
            lexopt::Error::UnexpectedValue { option, value } => {
                Self::UnexpectedValue { option, value }
            }
            lexopt::Error::ParsingFailed { value, error } => Self::ParsingFailed { value, error },
            lexopt::Error::NonUnicodeValue(s) => Self::NonUnicodeValue(s),
            lexopt::Error::Custom(e) => Self::Custom(e),
        }
    }
}

#[derive(Clone)]
pub enum Argument<T: Arguments> {
    Help,
    Version,
    Custom(T),
}

pub trait Arguments: Sized + Clone {
    fn parse<I>(args: I) -> ArgumentIter<Self>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        ArgumentIter::<Self>::from_args(args)
    }

    fn next_arg(
        parser: &mut lexopt::Parser,
        positional_idx: &mut usize,
    ) -> Result<Option<Argument<Self>>, Error>;

    fn check_missing(positional_idx: usize) -> Result<(), Error>;

    fn help(bin_name: &str) -> String;

    fn version() -> String;
}

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

    pub fn next_arg(&mut self) -> Result<Option<Argument<T>>, Error> {
        T::next_arg(&mut self.parser, &mut self.positional_idx)
    }

    pub fn help(&self) -> String {
        T::help(self.parser.bin_name().unwrap())
    }

    pub fn version(&self) -> String {
        T::version()
    }
}

pub trait Options: Sized + Default {
    fn parse<I>(args: I) -> Result<Self, Error>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        let mut _self = Self::default();
        _self.apply_args(args)?;
        Ok(_self)
    }

    fn apply_args<I>(&mut self, args: I) -> Result<(), Error>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>;
}

pub trait FromValue: Sized {
    fn from_value(value: OsString) -> Result<Self, lexopt::Error>;
}

impl FromValue for OsString {
    fn from_value(value: OsString) -> Result<Self, lexopt::Error> {
        Ok(value)
    }
}

impl FromValue for String {
    fn from_value(value: OsString) -> Result<Self, lexopt::Error> {
        Ok(value.into_string()?)
    }
}

impl<T> FromValue for Option<T>
where
    T: FromValue,
{
    fn from_value(value: OsString) -> Result<Self, lexopt::Error> {
        Ok(Some(T::from_value(value)?))
    }
}

macro_rules! from_value_int {
    ($t: ty) => {
        impl FromValue for $t {
            fn from_value(value: OsString) -> Result<Self, lexopt::Error> {
                let value = value.into_string()?;
                value
                    .parse()
                    .map_err(|e: ParseIntError| lexopt::Error::ParsingFailed {
                        value,
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

from_value_int!(i8);
from_value_int!(i16);
from_value_int!(i32);
from_value_int!(i64);
from_value_int!(i128);
