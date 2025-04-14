use uutils_args::{Arguments, Options};

struct Complicated {
    // Doesn't "impl Default". Oops!
}

#[derive(Arguments)]
enum Arg {
    #[arg("--foo")]
    Something(Complicated),
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
