pub use derive::*;
pub use lexopt;

use std::ffi::OsString;

pub trait Options: Sized {
    fn parse<I>(args: I) -> Result<Self, lexopt::Error>
    where
        I: IntoIterator + 'static,
        I::Item: Into<OsString>;
}
