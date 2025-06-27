// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Functions to be used by `uutils-args-derive`.
//!
//! This has the following implications:
//!  - These functions are not guaranteed to be stable.
//!  - These functions should not be used outside the derive crate
//!
//! Yet, they should be properly documented to make macro-expanded code
//! readable.

use crate::error::ErrorKind;
use crate::value::Value;
use std::{
    ffi::{OsStr, OsString},
    fmt::Write,
};

/// Parses an echo-style positional argument
///
/// This means that any argument that does not solely consist of a hyphen
/// followed by the characters in the list of `short_args` is considered
/// to be a positional argument, instead of an invalid argument. This
/// includes the `--` argument, which is ignored by `echo`.
pub fn echo_style_positional(p: &mut lexopt::Parser, short_args: &[char]) -> Option<OsString> {
    let mut raw = p.try_raw_args()?;
    let val = raw.peek()?;

    if is_echo_style_positional(val, short_args) {
        let val = val.into();
        raw.next();
        Some(val)
    } else {
        None
    }
}

fn is_echo_style_positional(s: &OsStr, short_args: &[char]) -> bool {
    let s = match s.to_str() {
        Some(x) => x,
        // If it's invalid utf-8 then it can't be a short arg, so must
        // be a positional argument.
        None => return true,
    };
    let mut chars = s.chars();
    let is_short_args = chars.next() == Some('-') && chars.all(|c| short_args.contains(&c));
    !is_short_args
}

/// Parse an argument defined by a prefix
pub fn parse_prefix<T: Value>(parser: &mut lexopt::Parser, prefix: &'static str) -> Option<T> {
    let mut raw = parser.try_raw_args()?;

    // TODO: The to_str call is a limitation. Maybe we need to pull in something like bstr
    let arg = raw.peek()?.to_str()?;
    let value_str = arg.strip_prefix(prefix)?;

    let value = T::from_value(OsStr::new(value_str)).ok()?;

    // Consume the argument we just parsed
    let _ = raw.next();

    Some(value)
}

/// Parse a value and wrap the error into an `Error::ParsingFailed`
pub fn parse_value_for_option<T: Value>(opt: &str, v: &OsStr) -> Result<T, ErrorKind> {
    T::from_value(v).map_err(|e| ErrorKind::ParsingFailed {
        option: opt.into(),
        value: v.to_string_lossy().to_string(),
        error: e,
    })
}

/// Expand unambiguous prefixes to a list of candidates
pub fn infer_long_option<'a>(
    input: &'a str,
    long_options: &'a [&'a str],
) -> Result<&'a str, ErrorKind> {
    let mut candidates = Vec::new();
    let mut exact_match = None;
    for opt in long_options {
        if *opt == input {
            exact_match = Some(opt);
            break;
        } else if opt.starts_with(input) {
            candidates.push(opt);
        }
    }

    match (exact_match, &candidates[..]) {
        (Some(opt), _) => Ok(*opt),
        (None, [opt]) => Ok(**opt),
        (None, []) => Err(ErrorKind::UnexpectedOption(
            format!("--{input}"),
            filter_suggestions(input, long_options, "--"),
        )),
        (None, _) => Err(ErrorKind::AmbiguousOption {
            option: input.to_string(),
            candidates: candidates.iter().map(|s| s.to_string()).collect(),
        }),
    }
}

/// Filter a list of options to just the elements that are similar to the given string
pub fn filter_suggestions(input: &str, long_options: &[&str], prefix: &str) -> Vec<String> {
    long_options
        .iter()
        .filter(|opt| strsim::jaro(input, opt) > 0.7)
        .map(|o| format!("{prefix}{o}"))
        .collect()
}

/// Print a formatted list of options.
pub fn print_flags(
    mut w: impl Write,
    indent_size: usize,
    width: usize,
    options: impl IntoIterator<Item = (&'static str, &'static str)>,
) {
    let indent = " ".repeat(indent_size);
    writeln!(w, "\nOptions:").unwrap();
    for (flags, help_string) in options {
        let mut help_lines = help_string.lines();
        write!(w, "{}{}", &indent, &flags).unwrap();

        if flags.len() <= width {
            let line = match help_lines.next() {
                Some(line) => line,
                None => {
                    writeln!(w).unwrap();
                    continue;
                }
            };
            let help_indent = " ".repeat(width - flags.len() + 2);
            writeln!(w, "{help_indent}{line}").unwrap();
        } else {
            writeln!(w).unwrap();
        }

        let help_indent = " ".repeat(width + indent_size + 2);
        for line in help_lines {
            writeln!(w, "{help_indent}{line}").unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsStr;

    use super::is_echo_style_positional;

    #[test]
    fn echo_positional() {
        assert!(is_echo_style_positional(OsStr::new("-aaa"), &['b']));
        assert!(is_echo_style_positional(OsStr::new("--"), &['b']));
        assert!(!is_echo_style_positional(OsStr::new("-b"), &['b']));
    }
}
