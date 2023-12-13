# Quick Start

A parser consists of two parts:

- an `enum` implementing [`Arguments`](crate::Arguments)
- an `struct` implementing [`Options`](crate::Options)

The `enum` defines all the arguments that your application accepts. The `struct` represents all configuration options for the application. In other words, the `struct` is the internal representation of the options, while the `enum` is the external representation.

## A single flag

We can create arguments by annotating a variant of an `enum` deriving [`Arguments`](crate::Arguments) with the `arg` attribute. This attribute takes strings that define the arguments. A short flag, for instance, looks like `"-f"` and a long flag looks like `"--flag"`. The full syntax for the arguments specifications can be found in the documentation for the [`Arguments` derive macro](derive@crate::Arguments)

To represent the program configuration we create a struct called `Settings`, which implements `Options<Arg>`. When an argument is encountered, we _apply_ it to the `Settings` struct. In this case, we set the `force` field of `Settings` to `true` if `Arg::Force` is parsed.

```rust
use uutils_args::{Arguments, Options};

#[derive(Arguments)]
enum Arg {
    #[arg("-f", "--force")]
    Force,
}

// Note: Debug, PartialEq and Eq are only necessary for assert_eq! below.
#[derive(Default, Debug, PartialEq, Eq)]
struct Settings {
    force: bool
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::Force => self.force = true,
        }
    }
}

assert_eq!(Settings::default().parse(["test"]), Settings { force: false });
assert_eq!(Settings::default().parse(["test", "-f"]), Settings { force: true });
```

## Two overriding flags

Of course, we can define multiple flags. If these arguments change the same fields of `Settings`, then they will override. This is important: by default none of the arguments will "conflict", they will always simply be processed in order.

```rust
use uutils_args::{Arguments, Options};

#[derive(Arguments)]
enum Arg {
    #[arg("-f", "--force")]
    Force,
    #[arg("-F", "--no-force")]
    NoForce,
}

// Note: Debug, PartialEq and Eq are only necessary for assert_eq! below.
#[derive(Default, Debug, PartialEq, Eq)]
struct Settings {
    force: bool
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::Force => self.force = true,
            Arg::NoForce => self.force = false,
        }
    }
}

assert_eq!(Settings::default().parse(["test"]), Settings { force: false });
assert_eq!(Settings::default().parse(["test", "-f"]), Settings { force: true });
assert_eq!(Settings::default().parse(["test", "-F"]), Settings { force: false });
```

## Help strings

We can document our flags in two ways: by giving them a docstring or by giving the `arg` attribute a `help` argument. Note that the `help` argument will take precedence over the docstring.

```rust
use uutils_args::Arguments;

#[derive(Arguments)]
enum Arg {
    /// Force!
    #[arg("-f", "--force")]
    Force,
    #[arg("-F", "--no-force", help = "No! Don't force!")]
    NoForce,
}
```

## Arguments with required values

So far, our arguments have been simple flags that do not take any arguments, but `uutils-args` supports much more! If we want an argument for our option, the corresponding variant on our `enum` needs to take an argument too.

> **Note**: In the example below, we use `OsString`. A regular `String` works too, but is generally discouraged in `coreutils`, because we often have to support text with invalid UTF-8.

```rust
# use uutils_args::{Arguments, Options};
# use std::ffi::OsString;
#
#[derive(Arguments)]
enum Arg {
    #[arg("-n NAME", "--name=NAME")]
    Name(OsString),
}
#
# // Note: Debug, PartialEq and Eq are only necessary for assert_eq! below.
# #[derive(Default, Debug, PartialEq, Eq)]
# struct Settings {
#     name: OsString
# }
#
# impl Options<Arg> for Settings {
#     fn apply(&mut self, arg: Arg) {
#         match arg {
#             Arg::Name(name) => self.name = name,
#         }
#     }
# }
#
# assert_eq!(
#     Settings::default().parse(["test"]),
#     Settings { name: OsString::new() }
# );
# assert_eq!(
#     Settings::default().parse(["test", "--name=John"]),
#     Settings { name: OsString::from("John")}
# );
```

## Arguments with optional values

Arguments with optional values are possible, too. However, we have to give a value to be used if the value is not given. Below, we set that value to `OsString::from("anonymous")`, with the `value` argument of `arg`.

```rust
# use uutils_args::{Arguments, Options};
# use std::ffi::OsString;
#
#[derive(Arguments)]
enum Arg {
    #[arg("-n[NAME]", "--name[=NAME]", value = OsString::from("anonymous"))]
    Name(OsString),
}
#
# #[derive(Default, Debug, PartialEq, Eq)]
# struct Settings {
#     name: OsString
# }
#
# impl Options<Arg> for Settings {
#     fn apply(&mut self, arg: Arg) {
#         match arg {
#             Arg::Name(name) => self.name = name,
#         }
#     }
# }
#
# assert_eq!(
#     Settings::default().parse(["test", "--name"]),
#     Settings { name: OsString::from("anonymous")}
# );
# assert_eq!(
#     Settings::default().parse(["test", "--name=John"]),
#     Settings { name: OsString::from("John")}
# );
```

## Multiple arguments per variant

Here's a neat trick: you can use multiple `arg` attributes per variant. Recall the `--force/--no-force` example above. We could have written that as follows:

```rust
# use uutils_args::{Arguments, Options};
#
#[derive(Arguments)]
enum Arg {
    #[arg("-f", "--force", value = true, help = "enable force")]
    #[arg("-F", "--no-force", value = false, help = "disable force")]
    Force(bool),
}
#
# #[derive(Default, Debug, PartialEq, Eq)]
# struct Settings {
#     force: bool
# }
#
# impl Options<Arg> for Settings {
#     fn apply(&mut self, arg: Arg) {
#         match arg {
#             Arg::Force(b) => self.force = b,
#         }
#     }
# }
#
# assert_eq!(Settings::default().parse(["test"]), Settings { force: false });
# assert_eq!(Settings::default().parse(["test", "-f"]), Settings { force: true });
# assert_eq!(Settings::default().parse(["test", "-F"]), Settings { force: false });
```

This is particularly interesting for defining "shortcut" arguments. For example, `ls` takes a `--sort=WORD` argument, that defines how the files should be sorted. But it also has shorthands like `-t`, which is the same as `--sort=time`. All of these can be implemented on one variant:

> **Note**: The `--sort` argument should not take a `String` as value. We've done that here for illustrative purposes. It should actually use an `enum` with the `Value` trait.

```rust
# use uutils_args::{Arguments, Options};
#
#[derive(Arguments)]
enum Arg {
    #[arg("--sort=WORD", help = "Sort by WORD")]
    #[arg("-t", value = String::from("time"), help = "Sort by time")]
    #[arg("-U", value = String::from("none"), help = "Do not sort")]
    #[arg("-v", value = String::from("version"), help = "Sort by version")]
    #[arg("-X", value = String::from("extension"), help = "Sort by extension")]
    Sort(String),
}
#
# #[derive(Default, Debug, PartialEq, Eq)]
# struct Settings {
#     sort: String
# }
#
# impl Options<Arg> for Settings {
#     fn apply(&mut self, arg: Arg) {
#         match arg {
#             Arg::Sort(s) => self.sort = s,
#         }
#     }
# }
#
# assert_eq!(Settings::default().parse(["test"]), Settings { sort: String::new() });
# assert_eq!(Settings::default().parse(["test", "--sort=time"]), Settings { sort: String::from("time") });
# assert_eq!(Settings::default().parse(["test", "-t"]), Settings { sort: String::from("time") });
```

## Positional arguments

Finally, at the end of this whirlwind tour, we get to positional arguments. Here's a simple positional argument:

```rust
use uutils_args::{Arguments, Options};
use std::path::PathBuf;

#[derive(Arguments)]
enum Arg {
    #[arg("FILES", ..)]
    File(PathBuf)
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Settings {
    files: Vec<PathBuf>,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::File(f) => self.files.push(f),
        }
    }
}
#
# assert_eq!(
#     Settings::default().parse(["test"]),
#     Settings { files: Vec::new() }
# );
# assert_eq!(
#    Settings::default().parse(["test", "foo"]),
#    Settings { files: vec![PathBuf::from("foo")] }
# );
# assert_eq!(
#     Settings::default().parse(["test", "foo", "bar"]),
#     Settings { files: vec!["foo".into(), "bar".into()] }
# );
```