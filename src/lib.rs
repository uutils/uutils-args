mod error;
pub use derive::*;
pub use lexopt;
pub use term_md;

pub use error::Error;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::{ffi::OsString, marker::PhantomData};

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
    fn parse<I>(args: I) -> Self
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        match Self::try_parse(args) {
            Ok(v) => v,
            Err(err) => {
                eprintln!("{err}");
                std::process::exit(0);
            }
        }
    }

    fn try_parse<I>(args: I) -> Result<Self, Error>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        let mut _self = Self::initial()?;
        _self.apply_args(args)?;
        Ok(_self)
    }

    fn initial() -> Result<Self, Error>;

    fn apply_args<I>(&mut self, args: I) -> Result<(), Error>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>;
}

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
