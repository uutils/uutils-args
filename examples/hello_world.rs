use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
#[help("--help")]
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
}

#[derive(Default, Options)]
#[arg_type(Arg)]
struct Settings {
    #[set(Arg::Name)]
    name: String,
    #[set(Arg::Count)]
    count: u8,
}

fn main() {
    let settings = Settings::parse(std::env::args_os()).unwrap();
    for _ in 0..settings.count {
        println!("Hello, {}!", settings.name);
    }
}
