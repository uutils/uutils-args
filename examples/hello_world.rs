use uutils_args::{Arguments, Options};

#[derive(Clone, Arguments)]
enum Arg {
    /// The name to greet
    #[option]
    Name(String),

    /// The number of times to greet
    #[option]
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
