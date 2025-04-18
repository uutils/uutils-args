use uutils_args::{Arguments, Options};

#[derive(Arguments)]
#[arguments(file = "nonexistent")] // Oops!
enum Arg {}

struct Settings {}

impl Options<Arg> for Settings {
    fn apply(&mut self, _arg: Arg) -> Result<(), uutils_args::Error> {
        Ok(())
    }
}

fn main() {
    Settings {}.parse(std::env::args_os()).unwrap();
}
