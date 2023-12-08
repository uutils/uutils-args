// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

mod fish;
mod zsh;

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

// Modelled after claps ValueHint
pub enum ValueHint {
    Strings(Vec<String>),
    Unknown,
    AnyPath,
    FilePath,
    DirPath,
    ExecutablePath,
    Username,
    Hostname,
}

pub fn render(c: &Command, shell: &str) -> String {
    match shell {
        "fish" => fish::render(c),
        "zsh" => zsh::render(c),
        "sh" | "bash" | "csh" | "elvish" | "powershell" => panic!("shell '{shell}' completion is not supported yet!"),
        _ => panic!("unknown shell '{shell}'!"),
    }
}
