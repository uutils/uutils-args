pub use derive::*;
pub use lexopt;

use std::ffi::OsString;

pub trait Options: Sized + Default {
    fn parse<I>(args: I) -> Result<Self, lexopt::Error>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>,
    {
        let mut _self = Self::default();
        _self.apply_args(args)?;
        Ok(_self)
    }

    fn apply_args<I>(&mut self, args: I) -> Result<(), lexopt::Error>
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
