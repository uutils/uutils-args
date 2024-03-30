use std::path::PathBuf;

use uutils_args::{Arguments, Options, Value};

#[derive(Value)]
enum Number {
    #[value]
    One,
    #[value]
    Two,
    #[value]
    Three,
}

#[derive(Arguments)]
enum Arg {
    /// Give it nothing!
    #[arg("-f", "--flag")]
    Flag,

    // Completion is derived from the `Number` type, through the `Value` trait
    /// Give it a number!
    #[arg("-n N", "--number=N")]
    Number(#[allow(unused)] Number),

    // Completion is derived from the `PathBuf` type
    /// Give it a path!
    #[arg("-p P", "--path=P")]
    Path(#[allow(unused)] PathBuf),
}

struct Settings;

impl Options<Arg> for Settings {
    fn apply(&mut self, _arg: Arg) -> Result<(), uutils_args::Error> {
        panic!("Compile with the 'parse-is-complete' feature!")
    }
}

fn main() {
    Settings.parse(std::env::args_os()).unwrap();
}
