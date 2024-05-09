use uutils_args::{Arguments, Options, Value};

#[derive(Arguments)]
#[arguments(file = "examples/hello_world_help.md")]
enum Arg {
    /// Color!
    #[arg("-c NAME", "--color=NAME")]
    Color(Color),
}

#[derive(Value, Debug, Default)]
enum Color {
    #[value("never")]
    Never,
    #[default]
    #[value("auto")]
    Auto,
    #[value("always")]
    Always,
}

#[derive(Default)]
struct Settings {
    color: Color,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::Color(c) => self.color = c,
        }
        Ok(())
    }
}

fn main() {
    let (settings, _operands) = Settings::default().parse(std::env::args_os()).unwrap();
    println!("{:?}", settings.color);
}
