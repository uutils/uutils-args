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
