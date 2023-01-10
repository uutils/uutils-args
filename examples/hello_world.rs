use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
#[arguments(
    help = ["--help"],
    version = ["--version"],
    file = "examples/hello_world_help.md"
)]
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

#[derive(Default, Options)]
#[arg_type(Arg)]
struct Settings {
    #[set(Arg::Name)]
    name: String,
    #[set(Arg::Count)]
    count: u8,
}

fn main() -> Result<(), uutils_args::Error> {
    let settings = Settings::parse(std::env::args_os());
    for _ in 0..settings.count {
        println!("Hello, {}!", settings.name);
    }
    Ok(())
}
