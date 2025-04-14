use uutils_args::{Arguments, Options, Value};

#[derive(Value, Default)]
enum Flavor {
    #[default]
    #[value(banana)] // Oops!
    Banana,
}

#[derive(Arguments)]
enum Arg {
    #[arg("--flavor=FLAVOR")]
    Flavor(Flavor),
}

struct Settings {}

impl Options<Arg> for Settings {
    fn apply(&mut self, _arg: Arg) -> Result<(), uutils_args::Error> {
        Ok(())
    }
}

fn main() {
    Settings {}.parse(std::env::args_os()).unwrap();
}
