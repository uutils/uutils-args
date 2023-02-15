use uutils_args::{Arguments, Initial, Options};

#[derive(Arguments)]
#[arguments(file = "examples/hello_world_help.md")]
enum Arg {
    /// The *name* to **greet**
    ///
    /// Just to show off, I can do multiple paragraphs and wrap text!
    ///
    /// # Also headings!
    #[option("-n NAME", "--name=NAME")]
    Name(String),

    /// The **number of times** to `greet`
    #[option("-c N", "--count=N")]
    Count(u8),

    /// This argument is hidden
    #[option("--hidden", hidden)]
    Hidden,
}

#[derive(Initial)]
struct Settings {
    name: String,
    #[field(default = 1)]
    count: u8,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::Name(n) => self.name = n,
            Arg::Count(c) => self.count = c,
            Arg::Hidden => {}
        }
    }
}

fn main() -> Result<(), uutils_args::Error> {
    let settings = Settings::parse(std::env::args_os());
    for _ in 0..settings.count {
        println!("Hello, {}!", settings.name);
    }
    Ok(())
}
