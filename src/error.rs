use std::{
    error::Error as StdError,
    ffi::OsString,
    fmt::{Debug, Display},
};

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
        option: String,
        value: String,
        error: Box<dyn StdError + Send + Sync + 'static>,
    },
    AmbiguousOption {
        option: String,
        candidates: Vec<String>,
    },
    AmbiguousValue {
        option: String,
        value: String,
        candidates: Vec<String>,
    },
    NonUnicodeValue(OsString),
    Custom(Box<dyn StdError + Send + Sync + 'static>),
}

impl StdError for Error {}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: ")?;
        match self {
            Error::MissingValue { option } => match option {
                Some(option) => write!(f, "Missing value for '{option}'."),
                None => write!(f, "Missing value"),
            },
            Error::MissingPositionalArguments(args) => {
                write!(f, "Missing values for the following positional arguments:")?;
                for arg in args {
                    write!(f, "  - {arg}")?;
                }
                Ok(())
            }
            Error::UnexpectedOption(opt) => {
                write!(f, "Found an invalid option '{opt}'.")
            }
            Error::UnexpectedArgument(arg) => {
                write!(f, "Found an invalid argument '{}'.", arg.to_string_lossy())
            }
            Error::UnexpectedValue { option, value } => {
                write!(
                    f,
                    "Got an unexpected value '{}' for option '{option}'.",
                    value.to_string_lossy(),
                )
            }
            Error::ParsingFailed {
                option,
                value,
                error,
            } => {
                if option.is_empty() {
                    write!(f, "Could not parse value '{value}': {error}")
                } else {
                    write!(
                        f,
                        "Could not parse value '{value}' for option '{option}': {error}"
                    )
                }
            }
            Error::AmbiguousOption { option, candidates } => {
                write!(
                    f,
                    "Option '{option}' is ambiguous. The following candidates match:"
                )?;
                for candidate in candidates {
                    write!(f, "  - {candidate}")?;
                }
                Ok(())
            }
            Error::AmbiguousValue {
                option,
                value,
                candidates,
            } => {
                write!(
                    f,
                    "Value '{value}' for option '{option}' is ambiguous. The following candidates match:",
                )?;
                for candidate in candidates {
                    write!(f, "  - {candidate}")?;
                }
                Ok(())
            }
            Error::NonUnicodeValue(x) => {
                write!(f, "Invalid unicode value found: {}", x.to_string_lossy())
            }
            Error::Custom(err) => std::fmt::Display::fmt(err, f),
        }
    }
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
            lexopt::Error::ParsingFailed { .. } => panic!("Conversion not supported"),
            lexopt::Error::NonUnicodeValue(s) => Self::NonUnicodeValue(s),
            lexopt::Error::Custom(e) => Self::Custom(e),
        }
    }
}
