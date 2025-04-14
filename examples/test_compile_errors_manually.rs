use uutils_args::{Arguments, Options, Value};

// Using a fully-fledged compile-error testsuite is a bit overkill, but we still
// want to make sure that the `derive` crate generates reasonable error messages.
// That's what this "example" is for. In the following, there are blocks of
// lines, one marked as POSITIVE and multiple lines marked as NEGATIVE. The
// committed version of this file should only contain POSITIVE. In order to run a
// test, comment out the POSITIVE line, and use a NEGATIVE line instead, and
// manually check whether you see a reasonable error message – ideally the error
// message indicated by the comment. One way to do this is:

// $ cargo build --example test_compile_errors_manually

#[derive(Value, Debug, Default)]
enum Flavor {
    #[default]
    #[value("kind", "nice")]
    Kind,
    #[value("condescending")] // POSITIVE
    // #[value(condescending)] // NEGATIVE: "expected comma-separated list of string literals"
    Condescending,
}

#[derive(Arguments)]
#[arguments(file = "examples/hello_world_help.md")] // POSITIVE
// #[arguments(file = "examples/nonexistent.md")] // NEGATIVE: "cannot open help-string file"
// #[arguments(file = "/dev/full")] // NEGATIVE: Causes OOM, FIXME
// #[arguments(file = "/")] // NEGATIVE: "cannot read from help-string file"
// #[arguments(file = "path/to/some/WRITE-ONLY/file")] // NEGATIVE: "cannot open help-string file"
enum Arg {
    /// The name to greet
    #[arg("-n NAME", "--name[=NAME]", "name=NAME")] // POSITIVE
    // #[arg("-")] // NEGATIVE: flag name must be non-empty (cannot be just '-')
    // #[arg("-n NAME", "--name[NAME]", "name=NAME")] // NEGATIVE: "expected '=' after '[' in flag pattern"
    // #[arg("-n NAME", "--name[=NAME", "name=NAME")] // NEGATIVE: "expected final ']' in flag pattern"
    // #[arg(key="name")] // NEGATIVE: "can't parse arg attributes, expected one or more strings"
    Name(String),

    /// The number of times to greet
    #[arg("-c N", "--count=N")]
    Count(u8),

    #[arg("--flavor=FLAVOR")]
    Flavor(Flavor),
}

struct Settings {
    name: String,
    count: u8,
    flavor: Flavor,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::Name(n) => self.name = n,
            Arg::Count(c) => self.count = c,
            Arg::Flavor(flavor) => self.flavor = flavor,
        }
        Ok(())
    }
}

fn main() -> Result<(), uutils_args::Error> {
    let (settings, _operands) = Settings {
        name: String::new(),
        count: 1,
        flavor: Flavor::Kind,
    }
    .parse(std::env::args_os())
    .unwrap();

    for _ in 0..settings.count {
        match settings.flavor {
            Flavor::Kind => {
                println!("Hello, {}!", settings.name);
            }
            Flavor::Condescending => {
                println!("Ugh, {}.", settings.name);
            }
        }
    }
    Ok(())
}
