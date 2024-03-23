use std::path::PathBuf;

use uutils_args::{Arguments, Options, Value};

#[derive(Value, Debug)]
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
    Number(Number),

    // Completion is derived from the `PathBuf` type
    /// Give it a path!
    #[arg("-p P", "--path=P")]
    Path(PathBuf),

    /// A dd_style argument!
    #[arg("if=file")]
    File(PathBuf),
}

struct Settings;

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::Flag => println!("Got flag"),
            Arg::Number(n) => println!("Got number {n:?}"),
            Arg::Path(p) => println!("Got path {}", p.display()),
            Arg::File(f) => println!("Got file {}", f.display()),
        }
    }
}

fn main() {
    Settings.parse(std::env::args_os()).unwrap();
}
