//! Parsing of positional arguments.
//!
//! The signature for parsing positional arguments is one of `&'static str`,
//! [`Opt`], [`Many0`], [`Many1`] or a tuple of those. The [`Unpack::unpack`]
//! method of these types parses a `Vec<T>` into the corresponding
//! [`Unpack::Output<T>`] type.
//!
//! For example:
//! ```
//! use std::ffi::OsString;
//! use uutils_args::positional::{Opt, Unpack};
//!
//! let (a, b) = ("FILE1", Opt("FILE2")).unpack(vec!["one"]).unwrap();
//! assert_eq!(a, "one");
//! assert_eq!(b, None);
//!
//! let (a, b) = ("FILE1", Opt("FILE2")).unpack(vec!["one", "two"]).unwrap();
//! assert_eq!(a, "one");
//! assert_eq!(b, Some("two"));
//!
//! // It works for any `Vec<T>`:
//! let (a, b) = ("FILE1", Opt("FILE2")).unpack(vec![1, 2]).unwrap();
//! assert_eq!(a, 1);
//! assert_eq!(b, Some(2));
//! ```
//!
//! Here are a few examples:
//!
//! ```ignore
//! ()               // no arguments
//! "FOO"            // one required argument with output `OsString`
//! Opt("FOO")       // one optional argument with output `Option<OsString>`
//! Many1("FOO")     // one or more arguments with output `Vec<OsString>`
//! Many0("FOO")     // zero or more arguments with output `Vec<OsString>`
//! ("FOO", "FOO")   // two required arguments with output (`OsString`, `OsString`)
//! ```
//!
//! This allows for the construction of complex signatures. The signature
//!
//! ```ignore
//! ("FOO", Many0("BAR"))
//! ```
//!
//! specifies that there is first a required argument "FOO" and any number of
//! values for "BAR".
//!
//! However, not all combinations are supported by design. For example, the
//! signature
//!
//! ```ignore
//! (Many0("FOO"), Many0("BAR"))
//! ```
//!
//! does not make sense, because it's unclear where the positional arguments
//! should go. The supported tuples implement [`Unpack`].

use crate::error::{Error, ErrorKind};
use std::fmt::Debug;

/// A required argument
type Req = &'static str;

/// Makes it's argument optional
pub struct Opt<T>(pub T);

/// 1 or more arguments
pub struct Many1(pub Req);

/// 0 or more arguments
pub struct Many0(pub Req);

/// Unpack a `Vec` into the output type
///
/// See the [module documentation](crate::positional) for more information.
pub trait Unpack {
    type Output<T>;
    fn unpack<T: Debug>(&self, operands: Vec<T>) -> Result<Self::Output<T>, Error>;
}

impl Unpack for () {
    type Output<T> = ();

    fn unpack<T: Debug>(&self, operands: Vec<T>) -> Result<Self::Output<T>, Error> {
        assert_empty(operands)
    }
}

impl<U: Unpack> Unpack for (U,) {
    type Output<T> = U::Output<T>;

    fn unpack<T: Debug>(&self, operands: Vec<T>) -> Result<Self::Output<T>, Error> {
        self.0.unpack(operands)
    }
}

impl Unpack for Req {
    type Output<T> = T;

    fn unpack<T: Debug>(&self, mut operands: Vec<T>) -> Result<Self::Output<T>, Error> {
        let arg = pop_front(self, &mut operands)?;
        assert_empty(operands)?;
        Ok(arg)
    }
}

impl<U: Unpack> Unpack for Opt<U> {
    type Output<T> = Option<U::Output<T>>;

    fn unpack<T: Debug>(&self, operands: Vec<T>) -> Result<Self::Output<T>, Error> {
        Ok(if operands.is_empty() {
            None
        } else {
            Some(self.0.unpack(operands)?)
        })
    }
}

impl Unpack for Many0 {
    type Output<T> = Vec<T>;

    fn unpack<T: Debug>(&self, operands: Vec<T>) -> Result<Self::Output<T>, Error> {
        Ok(operands)
    }
}

impl Unpack for Many1 {
    type Output<T> = Vec<T>;

    fn unpack<T: Debug>(&self, operands: Vec<T>) -> Result<Self::Output<T>, Error> {
        if operands.is_empty() {
            return Err(Error {
                exit_code: 1,
                kind: ErrorKind::MissingPositionalArguments(vec![self.0.into()]),
            });
        }
        Ok(operands)
    }
}

impl<U: Unpack> Unpack for (Req, U) {
    type Output<T> = (T, U::Output<T>);

    fn unpack<T: Debug>(&self, mut operands: Vec<T>) -> Result<Self::Output<T>, Error> {
        let arg = pop_front(self.0, &mut operands)?;
        let rest = self.1.unpack(operands)?;
        Ok((arg, rest))
    }
}

impl<U: Unpack> Unpack for (Req, Req, U) {
    type Output<T> = (T, T, U::Output<T>);

    fn unpack<T: Debug>(&self, mut operands: Vec<T>) -> Result<Self::Output<T>, Error> {
        let arg1 = pop_front(self.0, &mut operands)?;
        let arg2 = pop_front(self.1, &mut operands)?;
        let rest = self.2.unpack(operands)?;
        Ok((arg1, arg2, rest))
    }
}

impl<U: Unpack> Unpack for (Opt<U>, Req) {
    type Output<T> = (Option<<U as Unpack>::Output<T>>, T);

    fn unpack<T: Debug>(&self, mut operands: Vec<T>) -> Result<Self::Output<T>, Error> {
        let arg = pop_back(self.1, &mut operands)?;
        let rest = self.0.unpack(operands)?;
        Ok((rest, arg))
    }
}

impl Unpack for (Many0, Req) {
    type Output<T> = (Vec<T>, T);

    fn unpack<T: Debug>(&self, mut operands: Vec<T>) -> Result<Self::Output<T>, Error> {
        let arg = pop_back(self.1, &mut operands)?;
        let rest = self.0.unpack(operands)?;
        Ok((rest, arg))
    }
}

impl Unpack for (Many1, Req) {
    type Output<T> = (Vec<T>, T);

    fn unpack<T: Debug>(&self, mut operands: Vec<T>) -> Result<Self::Output<T>, Error> {
        let arg = pop_back(self.1, &mut operands)?;
        let rest = self.0.unpack(operands)?;
        Ok((rest, arg))
    }
}

fn pop_front<T: Debug>(name: &str, operands: &mut Vec<T>) -> Result<T, Error> {
    if operands.is_empty() {
        return Err(Error {
            exit_code: 1,
            kind: ErrorKind::MissingPositionalArguments(vec![name.to_string()]),
        });
    }
    Ok(operands.remove(0))
}

fn pop_back<T: Debug>(name: &str, operands: &mut Vec<T>) -> Result<T, Error> {
    operands.pop().ok_or_else(|| Error {
        exit_code: 1,
        kind: ErrorKind::MissingPositionalArguments(vec![name.to_string()]),
    })
}

fn assert_empty<T: Debug>(mut operands: Vec<T>) -> Result<(), Error> {
    if let Some(arg) = operands.pop() {
        return Err(Error {
            exit_code: 1,
            kind: ErrorKind::UnexpectedArgument(format!("{arg:?}")),
        });
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{Many0, Many1, Opt, Unpack};

    macro_rules! a {
        ($e:expr, $t:ty) => {
            let _: Result<$t, _> = $e.unpack(Vec::<&str>::new());
        };
    }

    #[track_caller]
    fn assert_ok<'a, U: Unpack, const N: usize>(
        signature: &U,
        expected: U::Output<&'a str>,
        operands: [&'a str; N],
    ) where
        U::Output<&'a str>: Eq + std::fmt::Debug,
    {
        assert_eq!(signature.unpack(Vec::from(operands)).unwrap(), expected);
    }

    #[track_caller]
    fn assert_err<const N: usize>(signature: &impl Unpack, operands: [&str; N]) {
        let operands = Vec::from(operands);
        assert!(signature.unpack(operands).is_err());
    }

    #[test]
    fn compile_tests() {
        // The five basic ones
        a!((), ());
        a!("FOO", &str);
        a!(Opt("FOO"), Option<&str>);
        a!(Many0("FOO"), Vec<&str>);
        a!(Many1("FOO"), Vec<&str>);

        // Start building some tuples
        a!(("FOO", "BAR"), (&str, &str));
        a!(("FOO", Opt("BAR")), (&str, Option<&str>));
        a!(("FOO", Many0("BAR")), (&str, Vec<&str>));
        a!(("FOO", Many1("BAR")), (&str, Vec<&str>));

        // The other way around!
        a!((Opt("FOO"), "BAR"), (Option<&str>, &str));
        a!((Many0("FOO"), "BAR"), (Vec<&str>, &str));
        a!((Many1("FOO"), "BAR"), (Vec<&str>, &str));

        // Longer tuples!
        a!(("FOO", "BAR", "BAZ"), (&str, &str, &str));
        a!(("FOO", "BAR", Opt("BAZ")), (&str, &str, Option<&str>));
        a!(("FOO", "BAR", Many0("BAZ")), (&str, &str, Vec<&str>));
        a!(("FOO", "BAR", Many1("BAZ")), (&str, &str, Vec<&str>));

        // seq [FIRST [INCREMENT]] LAST
        a!(
            (Opt(("FIRST", Opt("INCREMENT"))), "LAST"),
            (Option<(&str, Option<&str>)>, &str)
        );

        // mknod NAME TYPE [MAJOR MINOR]
        a!(
            ("NAME", "TYPE", Opt(("MAJOR", "MINOR"))),
            (&str, &str, Option<(&str, &str)>)
        );

        // chroot
        a!(
            ("NEWROOT", Opt(("COMMAND", Many0("ARG")))),
            (&str, Option<(&str, Vec<&str>)>)
        );
    }

    #[test]
    fn unit() {
        assert_ok(&(), (), []);
        assert_err(&(), ["foo"]);
        assert_err(&(), ["foo", "bar"]);
    }

    #[test]
    fn required() {
        let s = "FOO";
        assert_err(&s, []);
        assert_ok(&s, "foo", ["foo"]);
        assert_err(&s, ["foo", "bar"]);
        assert_err(&s, ["foo", "bar", "baz"]);
    }

    #[test]
    fn optional() {
        let s = Opt("FOO");
        assert_ok(&s, None, []);
        assert_ok(&s, Some("foo"), ["foo"]);
        assert_err(&s, ["foo", "bar"]);
        assert_err(&s, ["foo", "bar", "baz"]);
    }

    #[test]
    fn many1() {
        let s = Many1("FOO");
        assert_err(&s, []);
        assert_ok(&s, vec!["foo"], ["foo"]);
        assert_ok(&s, vec!["foo", "bar"], ["foo", "bar"]);
        assert_ok(&s, vec!["foo", "bar", "baz"], ["foo", "bar", "baz"]);
    }

    #[test]
    fn many0() {
        let s = Many0("FOO");
        assert_ok(&s, vec![], []);
        assert_ok(&s, vec!["foo"], ["foo"]);
        assert_ok(&s, vec!["foo", "bar"], ["foo", "bar"]);
        assert_ok(&s, vec!["foo", "bar", "baz"], ["foo", "bar", "baz"]);
    }

    #[test]
    fn req_req() {
        let s = ("FOO", "BAR");
        assert_err(&s, []);
        assert_err(&s, ["foo"]);
        assert_ok(&s, ("foo", "bar"), ["foo", "bar"]);
        assert_err(&s, ["foo", "bar", "baz"]);
    }

    #[test]
    fn seq() {
        let s = (Opt(("FIRST", Opt("INCREMENT"))), "LAST");
        assert_err(&s, []);
        assert_ok(&s, (None, "1"), ["1"]);
        assert_ok(&s, (Some(("1", None)), "2"), ["1", "2"]);
        assert_ok(&s, (Some(("1", Some("2"))), "3"), ["1", "2", "3"]);
        assert_err(&s, ["1", "2", "3", "4"]);
    }

    #[test]
    fn mknod() {
        let s = ("NAME", "TYPE", Opt(("MAJOR", "MINOR")));
        assert_err(&s, []);
        assert_err(&s, ["1"]);
        assert_ok(&s, ("1", "2", None), ["1", "2"]);
        assert_err(&s, ["1", "2", "3"]);
        assert_ok(&s, ("1", "2", Some(("3", "4"))), ["1", "2", "3", "4"]);
    }
}
