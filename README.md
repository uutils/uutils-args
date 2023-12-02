# uutils-args

Argument parsing for the [uutils coreutils](https://www.github.com/uutils/coreutils) project.

It is designed to be flexible, while providing default
behaviour that aligns with GNU coreutils.

## Features

 - A derive macro for declarative argument definition.
 - Automatic help generation.
 - Positional and optional arguments.
 - Automatically parsing values into Rust types.
 - Define a custom exit code on errors.
 - Automatically accept unambiguous abbreviations of long options.
 - Handles invalid UTF-8 gracefully.

## When you should not use this library

The goal of this library is to make it easy to build applications that
mimic the behaviour of the GNU coreutils. There are other applications
that have similar behaviour, which are C application that use `getopt`
and `getopt_long`. If you want to mimic that behaviour exactly, this
is the library for you. If you want to write basically anything else,
you should probably pick another argument parser.

## Getting Started

Parsing with this library consists of two "phases". In the first
phase, the arguments are mapped to an iterator of an `enum`
implementing [`Arguments`]. The second phase is mapping these
arguments onto a `struct` implementing [`Options`]. By defining
your arguments this way, there is a clear divide between the public
API and the internal representation of the settings of your app.

For more information on these traits, see their respective documentation:

- [`Arguments`]
- [`Options`]

Below is a minimal example of a full CLI application using this library.

```rust
use uutils_args::{Arguments, Initial, Options};

#[derive(Arguments)]
enum Arg {
    // The doc strings below will be part of the `--help` text
    // First we define a simple flag:
    /// Do not transform input text to uppercase
    #[option("-n", "--no-caps")]
    NoCaps,
    
    // This option takes a value:    
    /// Add exclamation marks to output
    #[option("-e N", "--exclaim=N")]
    ExclamationMarks(u8),

    // This is a positional argument, the range specifies that
    // at least one positional argument must be passed.
    #[positional(1..)]
    Text(String),
}

#[derive(Initial)]
struct Settings {
    // We can change the default value with the field attribute.
    #[initial(true)]
    caps: bool,
    exclamation_marks: u8,
    text: String,
}

// To implement `Options`, we only need to provide the `apply` method.
// The `parse` method will be automatically generated.
impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::NoCaps => self.caps = false,
            Arg::ExclamationMarks(n) => self.exclamation_marks += n,
            Arg::Text(s) => {
                if self.text.is_empty() {
                    self.text.push_str(&s);
                } else {
                    self.text.push(' ');
                    self.text.push_str(&s);
                }
            }
        }
    }
}

fn run(args: &'static [&'static str]) -> String {
    let s = Settings::parse(args);
    let mut output = if s.caps {
        s.text.to_uppercase()
    } else {
        s.text
    };
    for i in 0..s.exclamation_marks {
        output.push('!');
    }
    output
}

// The first argument is the binary name. In this example it's ignored.
assert_eq!(run(&["shout", "hello"]), "HELLO");
assert_eq!(run(&["shout", "-e3", "hello"]), "HELLO!!!");
assert_eq!(run(&["shout", "-e", "3", "hello"]), "HELLO!!!");
assert_eq!(run(&["shout", "--no-caps", "hello"]), "hello");
assert_eq!(run(&["shout", "-e3", "-n", "hello"]), "hello!!!");
assert_eq!(run(&["shout", "-e3", "hello", "world"]), "HELLO WORLD!!!");
```

## Additional functionality

To make it easier to implement [`Arguments`] and [`Options`], there are
two additional traits:

- [`Initial`] is an alternative to the [`Default`] trait from the standard
  library, with a richer derive macro.
- [`Value`] allows for easy parsing from `OsStr` to any type
  implementing [`Value`]. This crate also provides a derive macro for
  this trait.

## Examples

The following files contain examples of commands defined with
`uutils_args`:

- [hello world](https://github.com/uutils/uutils-args/blob/main/examples/hello_world.rs)
- [arch](https://github.com/uutils/uutils-args/blob/main/tests/coreutils/arch.rs)
- [b2sum](https://github.com/uutils/uutils-args/blob/main/tests/coreutils/b2sum.rs)
- [base32](https://github.com/uutils/uutils-args/blob/main/tests/coreutils/base32.rs)
- [basename](https://github.com/uutils/uutils-args/blob/main/tests/coreutils/basename.rs)
- [cat](https://github.com/uutils/uutils-args/blob/main/tests/coreutils/cat.rs)
- [echo](https://github.com/uutils/uutils-args/blob/main/tests/coreutils/echo.rs)
- [ls](https://github.com/uutils/uutils-args/blob/main/tests/coreutils/ls.rs)
- [mktemp](https://github.com/uutils/uutils-args/blob/main/tests/coreutils/mktemp.rs)
