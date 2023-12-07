// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

mod fish;

pub struct Command {
    pub name: String,
    pub args: Vec<Arg>,
}

#[derive(Default)]
pub struct Arg {
    pub short: Vec<String>,
    pub long: Vec<String>,
    pub help: String,
    pub value: Option<ValueHint>,
}

pub enum ValueHint {
    Strings(Vec<String>),
    Unknown,
    // Other,
    AnyPath,
    // FilePath,
    // DirPath,
    // ExecutablePath,
    // CommandName,
    // CommandString,
    // CommandWithArguments,
    // Username,
    // Hostname,
}

pub fn render(c: &Command, shell: &str) -> String {
    match shell {
        "fish" => fish::render(c),
        "sh" | "zsh" | "bash" | "csh" => panic!("shell '{shell}' completion is not supported yet!"),
        _ => panic!("unknown shell '{shell}'!"),
    }
}
