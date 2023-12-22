//! Parsing of positional arguments.
//!
//! The signature for parsing positional arguments is one of [`&'static str`],
//! [`Opt`], [`Many`] or a tuple of those. The [`Unpack::unpack`] method of
//! these types parses a `Vec<OsString>` into the corresponding
//! [`Unpack::Output`] type.
//!
//! For example:
//! ```
//! use std::ffi::OsString;
//! use uutils_args::positional::{Opt, Unpack};
//!
//! let (a, b) = ("FILE1", Opt("FILE2")).unpack(vec![OsString::from("one")]).unwrap();
//! assert_eq!(a, OsString::from("one"));
//! assert_eq!(b, None);
//!
//! let (a, b) = ("FILE1", Opt("FILE2")).unpack(vec![OsString::from("one"), OsString::from("two")]).unwrap();
//! assert_eq!(a, OsString::from("one"));
//! assert_eq!(b, Some(OsString::from("two")));
//! ```
//!
//! Here are a few examples:
//!
//! ```ignore
//! ()               // no arguments
//! "FOO"            // one required argument with output `OsString`
//! Opt("FOO")       // one optional argument with output `Option<OsString>`
//! Many("FOO")      // one or more arguments with output `Vec<OsString>`
//! Opt(Many("FOO")) // zero or more arguments with output `Vec<OsString>`
//! ("FOO", "FOO")   // two required arguments with output (`OsString`, `OsString`)
//! ```
//!
//! This allows for the construction of complex signatures. The signature
//!
//! ```ignore
//! ("FOO", Opt(Many("BAR")))
//! ```
//!
//! specifies that there is first a required argument "FOO" and any number of
//! values for "BAR".
//!
//! However, not all combinations are supported by design. For example, the
//! signature
//!
//! ```ignore
//! (Many("FOO"), Many("BAR"))
//! ```
//!
//! does not make sense, because it's unclear where the positional arguments
//! should go. The supported tuples implement [`Unpack`].

use crate::error::{Error, ErrorKind};
use std::ffi::OsString;

/// A required argument
type Req = &'static str;

/// Makes it's argument optional
pub struct Opt<T>(pub T);

/// 1 or more arguments
pub struct Many(pub Req);

/// Unpack a `Vec` into the output type
///
/// See the [module documentation](crate::positional) for more information.
pub trait Unpack {
    type Output: ToOptional;
    fn unpack(&self, operands: Vec<OsString>) -> Result<Self::Output, Error>;
}

impl Unpack for () {
    type Output = ();

    fn unpack(&self, operands: Vec<OsString>) -> Result<Self::Output, Error> {
        assert_empty(operands)
    }
}

impl<T: Unpack> Unpack for (T,) {
    type Output = T::Output;

    fn unpack(&self, operands: Vec<OsString>) -> Result<Self::Output, Error> {
        self.0.unpack(operands)
    }
}

impl Unpack for Req {
    type Output = OsString;

    fn unpack(&self, mut operands: Vec<OsString>) -> Result<Self::Output, Error> {
        let arg = pop_front(self, &mut operands)?;
        assert_empty(operands)?;
        Ok(arg)
    }
}

impl<T: Unpack> Unpack for Opt<T>
where
    T::Output: ToOptional,
{
    type Output = <T::Output as ToOptional>::Out;

    fn unpack(&self, operands: Vec<OsString>) -> Result<Self::Output, Error> {
        Ok(if operands.is_empty() {
            <T::Output as ToOptional>::none()
        } else {
            self.0.unpack(operands)?.some()
        })
    }
}

impl Unpack for Many {
    type Output = Vec<OsString>;

    fn unpack(&self, operands: Vec<OsString>) -> Result<Self::Output, Error> {
        if operands.is_empty() {
            return Err(Error {
                exit_code: 1,
                kind: ErrorKind::MissingPositionalArguments(vec![self.0.into()]),
            });
        }
        Ok(operands)
    }
}

impl<T: Unpack> Unpack for (Req, T) {
    type Output = (OsString, T::Output);

    fn unpack(&self, mut operands: Vec<OsString>) -> Result<Self::Output, Error> {
        let arg = pop_front(self.0, &mut operands)?;
        let rest = self.1.unpack(operands)?;
        Ok((arg, rest))
    }
}

impl<T: Unpack> Unpack for (Req, Req, T) {
    type Output = (OsString, OsString, T::Output);

    fn unpack(&self, mut operands: Vec<OsString>) -> Result<Self::Output, Error> {
        let arg1 = pop_front(self.0, &mut operands)?;
        let arg2 = pop_front(self.1, &mut operands)?;
        let rest = self.2.unpack(operands)?;
        Ok((arg1, arg2, rest))
    }
}

impl<T: Unpack> Unpack for (Opt<T>, Req) {
    type Output = (<Opt<T> as Unpack>::Output, <Req as Unpack>::Output);

    fn unpack(&self, mut operands: Vec<OsString>) -> Result<Self::Output, Error> {
        let arg = pop_back(self.1, &mut operands)?;
        let rest = self.0.unpack(operands)?;
        Ok((rest, arg))
    }
}

impl Unpack for (Many, Req) {
    type Output = (<Many as Unpack>::Output, <Req as Unpack>::Output);

    fn unpack(&self, mut operands: Vec<OsString>) -> Result<Self::Output, Error> {
        let arg = pop_back(self.1, &mut operands)?;
        let rest = self.0.unpack(operands)?;
        Ok((rest, arg))
    }
}

fn pop_front(name: &str, operands: &mut Vec<OsString>) -> Result<OsString, Error> {
    if operands.is_empty() {
        return Err(Error {
            exit_code: 1,
            kind: ErrorKind::MissingPositionalArguments(vec![name.into()]),
        });
    }
    Ok(operands.remove(0))
}

fn pop_back(name: &str, operands: &mut Vec<OsString>) -> Result<OsString, Error> {
    operands.pop().ok_or_else(|| Error {
        exit_code: 1,
        kind: ErrorKind::MissingPositionalArguments(vec![name.into()]),
    })
}

fn assert_empty(mut operands: Vec<OsString>) -> Result<(), Error> {
    if let Some(arg) = operands.pop() {
        return Err(Error {
            exit_code: 1,
            kind: ErrorKind::UnexpectedArgument(arg),
        });
    }
    Ok(())
}

pub trait ToOptional {
    type Out: ToOptional;
    fn some(self) -> Self::Out;
    fn none() -> Self::Out;
}

impl ToOptional for OsString {
    type Out = Option<Self>;
    fn some(self) -> Self::Out {
        Some(self)
    }
    fn none() -> Self::Out {
        None
    }
}

impl ToOptional for () {
    type Out = Option<Self>;
    fn some(self) -> Self::Out {
        Some(self)
    }
    fn none() -> Self::Out {
        None
    }
}

impl<T> ToOptional for Vec<T> {
    type Out = Self;
    fn some(self) -> Self::Out {
        self
    }
    fn none() -> Self::Out {
        Vec::new()
    }
}

impl<T1, T2> ToOptional for (T1, T2) {
    type Out = Option<Self>;
    fn some(self) -> Self::Out {
        Some(self)
    }
    fn none() -> Self::Out {
        None
    }
}

impl<T1, T2, T3> ToOptional for (T1, T2, T3) {
    type Out = Option<Self>;
    fn some(self) -> Self::Out {
        Some(self)
    }
    fn none() -> Self::Out {
        None
    }
}

impl<T1> ToOptional for Option<T1> {
    type Out = Self;
    fn some(self) -> Self::Out {
        self
    }
    fn none() -> Self::Out {
        None
    }
}

#[cfg(test)]
mod test {
    use super::{Many, Opt, Unpack};
    use std::ffi::OsString;

    macro_rules! a {
        ($e:expr, $t:ty) => {
            let _: Result<$t, _> = $e.unpack(Vec::new());
        };
    }

    #[track_caller]
    fn assert_ok<U: Unpack, const N: usize>(signature: &U, expected: U::Output, operands: [&str; N])
    where
        U::Output: Eq + std::fmt::Debug,
    {
        let operands = operands.into_iter().map(Into::into).collect();
        assert_eq!(signature.unpack(operands).unwrap(), expected);
    }

    #[track_caller]
    fn assert_err<const N: usize>(signature: &impl Unpack, operands: [&str; N]) {
        let operands = operands.into_iter().map(Into::into).collect();
        assert!(signature.unpack(operands).is_err());
    }

    #[test]
    fn compile_tests() {
        // The five basic ones
        a!((), ());
        a!("FOO", OsString);
        a!(Opt("FOO"), Option<OsString>);
        a!(Many("FOO"), Vec<OsString>);
        a!(Opt(Many("FOO")), Vec<OsString>);

        // Start building some tuples
        a!(("FOO", "BAR"), (OsString, OsString));
        a!(("FOO", Opt("BAR")), (OsString, Option<OsString>));
        a!(("FOO", Many("BAR")), (OsString, Vec<OsString>));
        a!(("FOO", Opt(Many("BAR"))), (OsString, Vec<OsString>));

        // The other way around!
        a!((Opt("FOO"), "BAR"), (Option<OsString>, OsString));
        a!((Many("FOO"), "BAR"), (Vec<OsString>, OsString));
        a!((Opt(Many("FOO")), "BAR"), (Vec<OsString>, OsString));

        // Longer tuples!
        a!(("FOO", "BAR", "BAZ"), (OsString, OsString, OsString));
        a!(
            ("FOO", "BAR", Opt("BAZ")),
            (OsString, OsString, Option<OsString>)
        );
        a!(
            ("FOO", "BAR", Many("BAZ")),
            (OsString, OsString, Vec<OsString>)
        );
        a!(
            ("FOO", "BAR", Opt(Many("BAZ"))),
            (OsString, OsString, Vec<OsString>)
        );

        // seq [FIRST [INCREMENT]] LAST
        a!(
            (Opt(("FIRST", Opt("INCREMENT"))), "LAST"),
            (Option<(OsString, Option<OsString>)>, OsString)
        );

        // mknod NAME TYPE [MAJOR MINOR]
        a!(
            ("NAME", "TYPE", Opt(("MAJOR", "MINOR"))),
            (OsString, OsString, Option<(OsString, OsString)>)
        );

        // chroot
        a!(
            ("NEWROOT", Opt(("COMMAND", Opt(Many("ARG"))))),
            (OsString, Option<(OsString, Vec<OsString>)>)
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
        assert_ok(&s, "foo".into(), ["foo"]);
        assert_err(&s, ["foo", "bar"]);
        assert_err(&s, ["foo", "bar", "baz"]);
    }

    #[test]
    fn optional() {
        let s = Opt("FOO");
        assert_ok(&s, None, []);
        assert_ok(&s, Some("foo".into()), ["foo"]);
        assert_err(&s, ["foo", "bar"]);
        assert_err(&s, ["foo", "bar", "baz"]);
    }

    #[test]
    fn many() {
        let s = Many("FOO");
        assert_err(&s, []);
        assert_ok(&s, vec!["foo".into()], ["foo"]);
        assert_ok(&s, vec!["foo".into(), "bar".into()], ["foo", "bar"]);
        assert_ok(
            &s,
            vec!["foo".into(), "bar".into(), "baz".into()],
            ["foo", "bar", "baz"],
        );
    }

    #[test]
    fn opt_many() {
        let s = Opt(Many("FOO"));
        assert_ok(&s, vec![], []);
        assert_ok(&s, vec!["foo".into()], ["foo"]);
        assert_ok(&s, vec!["foo".into(), "bar".into()], ["foo", "bar"]);
        assert_ok(
            &s,
            vec!["foo".into(), "bar".into(), "baz".into()],
            ["foo", "bar", "baz"],
        );
    }

    #[test]
    fn req_req() {
        let s = ("FOO", "BAR");
        assert_err(&s, []);
        assert_err(&s, ["foo"]);
        assert_ok(&s, ("foo".into(), "bar".into()), ["foo", "bar"]);
        assert_err(&s, ["foo", "bar", "baz"]);
    }
}
