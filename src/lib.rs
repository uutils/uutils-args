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

fn exit_if_err<T>(res: Result<T, Error>, exit_code: i32) -> T {
    match res {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(exit_code);
        }
    }
}

pub trait Arguments: Sized {
    const EXIT_CODE: i32;

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

    fn check<I>(args: I)
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        exit_if_err(Self::try_check(args), Self::EXIT_CODE)
    }

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

pub trait Initial: Sized {
    fn initial() -> Result<Self, Error>;
}

pub trait Options: Sized + Initial {
    type Arg: Arguments;

    fn apply(&mut self, arg: Self::Arg);

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
        let mut _self = Self::initial()?;
        let mut iter = Self::Arg::parse(args);
        while let Some(arg) = iter.next_arg()? {
            _self.apply(arg);
        }
        Self::Arg::check_missing(iter.positional_idx)?;
        Ok(_self)
    }
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
