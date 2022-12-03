pub use derive::*;
pub use lexopt;
use std::error::Error as StdError;

use std::{ffi::OsString, marker::PhantomData};

#[derive(Debug)]
pub enum Error {
    MissingValue {
        option: Option<String>,
    },
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

pub trait Arguments: Sized {
    fn parse<I>(args: I) -> ArgumentIter<Self>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        ArgumentIter::<Self>::from_args(args)
    }

    fn next_arg(parser: &mut lexopt::Parser) -> Result<Option<Self>, Error>;
}

pub struct ArgumentIter<T: Arguments> {
    parser: lexopt::Parser,
    t: PhantomData<T>,
}

impl<T: Arguments> ArgumentIter<T> {
    fn from_args<I>(args: I) -> Self
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        Self {
            parser: lexopt::Parser::from_args(args),
            t: PhantomData,
        }
    }

    pub fn next_arg(&mut self) -> Result<Option<T>, Error> {
        T::next_arg(&mut self.parser)
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
