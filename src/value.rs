// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::error::{Error, ErrorKind};
use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};
#[cfg(feature = "complete")]
use uutils_args_complete::ValueHint;

pub type ValueResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub enum ValueError {
    /// An abbreviated value was given that could match multiple values.
    AmbiguousValue {
        value: String,
        candidates: Vec<String>,
    },
    InvalidUnicode(OsString),
}

impl std::error::Error for ValueError {}

impl std::fmt::Debug for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueError::AmbiguousValue { value, candidates } => {
                write!(
                    f,
                    "Value '{value}' is ambiguous. The following candidates match:"
                )?;
                for candidate in candidates {
                    write!(f, "  - {candidate}")?;
                }
                Ok(())
            }
            ValueError::InvalidUnicode(x) => {
                write!(f, "'{}' is invalid unicode.", x.to_string_lossy())
            }
        }
    }
}

/// Defines how a type should be parsed from an argument.
///
/// If an error is returned, it will be wrapped in [`ErrorKind::ParsingFailed`]
pub trait Value: Sized {
    fn from_value(value: &OsStr) -> ValueResult<Self>;

    #[cfg(feature = "complete")]
    fn value_hint() -> ValueHint {
        ValueHint::Unknown
    }
}

impl Value for OsString {
    fn from_value(value: &OsStr) -> ValueResult<Self> {
        Ok(value.into())
    }
}

impl Value for PathBuf {
    fn from_value(value: &OsStr) -> ValueResult<Self> {
        Ok(PathBuf::from(value))
    }

    #[cfg(feature = "complete")]
    fn value_hint() -> ValueHint {
        ValueHint::AnyPath
    }
}

impl Value for String {
    fn from_value(value: &OsStr) -> ValueResult<Self> {
        match value.to_str() {
            Some(s) => Ok(s.into()),
            None => Err(Error {
                exit_code: 1,
                kind: ErrorKind::NonUnicodeValue(value.into()),
            }
            .into()),
        }
    }
}

impl<T> Value for Option<T>
where
    T: Value,
{
    fn from_value(value: &OsStr) -> ValueResult<Self> {
        Ok(Some(T::from_value(value)?))
    }

    #[cfg(feature = "complete")]
    fn value_hint() -> uutils_args_complete::ValueHint {
        T::value_hint()
    }
}

macro_rules! value_int {
    ($t: ty) => {
        impl Value for $t {
            fn from_value(value: &OsStr) -> ValueResult<Self> {
                let string = String::from_value(value)?;
                Ok(string.parse()?)
            }
        }
    };
}

value_int!(u8);
value_int!(u16);
value_int!(u32);
value_int!(u64);
value_int!(u128);
value_int!(usize);

value_int!(i8);
value_int!(i16);
value_int!(i32);
value_int!(i64);
value_int!(i128);
value_int!(isize);
