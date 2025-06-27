// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::{
    error::Error as StdError,
    ffi::OsString,
    fmt::{Debug, Display},
};

pub struct Error {
    pub exit_code: i32,
    pub kind: ErrorKind,
}

/// Errors that can occur while parsing arguments.
pub enum ErrorKind {
    /// There was an option that required an option, but none was given.
    MissingValue {
        option: Option<String>,
    },

    /// Some positional arguments were not given.
    MissingPositionalArguments(Vec<String>),

    /// An unrecognized option was passed.
    ///
    /// The second argument is a list of suggestions
    UnexpectedOption(String, Vec<String>),

    /// No more positional arguments were expected, but one was given anyway.
    UnexpectedArgument(String),

    /// A value was passed to an option that didn't expect a value.
    UnexpectedValue {
        option: String,
        value: OsString,
    },

    /// Parsing of a value failed.
    ParsingFailed {
        option: String,
        value: String,
        error: Box<dyn StdError + Send + Sync + 'static>,
    },

    /// An abbreviated long option was given that could match multiple
    /// long options.
    AmbiguousOption {
        option: String,
        candidates: Vec<String>,
    },

    /// The value was required to be valid UTF-8, but it wasn't.
    NonUnicodeValue(OsString),

    IoError(std::io::Error),
}

impl From<std::io::Error> for ErrorKind {
    fn from(value: std::io::Error) -> Self {
        ErrorKind::IoError(value)
    }
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.kind, f)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: ")?;
        match self {
            ErrorKind::MissingValue { option } => match option {
                Some(option) => write!(f, "Missing value for '{option}'."),
                None => write!(f, "Missing value"),
            },
            ErrorKind::MissingPositionalArguments(args) => {
                write!(f, "Missing values for the following positional arguments:")?;
                for arg in args {
                    write!(f, "  - {arg}")?;
                }
                Ok(())
            }
            ErrorKind::UnexpectedOption(opt, suggestions) => {
                write!(f, "Found an invalid option '{opt}'.")?;
                if !suggestions.is_empty() {
                    write!(f, "\nDid you mean: {}", suggestions.join(", "))?;
                }
                Ok(())
            }
            ErrorKind::UnexpectedArgument(arg) => {
                write!(f, "Found an invalid argument '{arg}'.")
            }
            ErrorKind::UnexpectedValue { option, value } => {
                write!(
                    f,
                    "Got an unexpected value '{}' for option '{option}'.",
                    value.to_string_lossy(),
                )
            }
            ErrorKind::ParsingFailed {
                option,
                value,
                error,
            } => {
                // TODO: option should not not be Option<String>, because even for positional
                // arguments we want to specify the name of the value.
                if option.is_empty() {
                    write!(f, "Invalid value '{value}': {error}")
                } else {
                    write!(f, "Invalid value '{value}' for '{option}': {error}")
                }
            }
            ErrorKind::AmbiguousOption { option, candidates } => {
                write!(
                    f,
                    "Option '{option}' is ambiguous. The following candidates match:"
                )?;
                for candidate in candidates {
                    write!(f, "  - {candidate}")?;
                }
                Ok(())
            }
            ErrorKind::NonUnicodeValue(x) => {
                write!(f, "Invalid unicode value found: {}", x.to_string_lossy())
            }
            ErrorKind::IoError(x) => std::fmt::Display::fmt(x, f),
        }
    }
}

impl From<lexopt::Error> for ErrorKind {
    fn from(other: lexopt::Error) -> ErrorKind {
        match other {
            lexopt::Error::MissingValue { option } => Self::MissingValue { option },
            lexopt::Error::UnexpectedOption(s) => Self::UnexpectedOption(s, Vec::new()),
            lexopt::Error::UnexpectedArgument(s) => {
                Self::UnexpectedArgument(s.to_string_lossy().to_string())
            }
            lexopt::Error::UnexpectedValue { option, value } => {
                Self::UnexpectedValue { option, value }
            }
            lexopt::Error::NonUnicodeValue(s) => Self::NonUnicodeValue(s),
            lexopt::Error::ParsingFailed { .. } | lexopt::Error::Custom(_) => {
                panic!("Should never be constructed.")
            }
        }
    }
}
