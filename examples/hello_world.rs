use uutils_args::{Arguments, Options};

#[derive(Arguments)]
#[arguments(file = "examples/hello_world_help.md")]
enum Arg {
    /// The name to greet
    #[arg("-n NAME", "--name=NAME", "name=NAME")]
    Name(String),

    /// The number of times to greet
    #[arg("-c N", "--count=N")]
    Count(u8),

    /// This argument is hidden
    #[arg("--hidden", hidden)]
    Hidden,
}

struct Settings {
    name: String,
    count: u8,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::Name(n) => self.name = n,
            Arg::Count(c) => self.count = c,
            Arg::Hidden => {}
        }
        Ok(())
    }
}

fn main() -> Result<(), uutils_args::Error> {
    let (settings, _operands) = Settings {
        name: String::new(),
        count: 1,
    }
    .parse(std::env::args_os())
    .unwrap();

    for _ in 0..settings.count {
        println!("Hello, {}!", settings.name);
    }
    Ok(())
}
