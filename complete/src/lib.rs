// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

mod fish;
mod man;
mod md;
mod zsh;

#[derive(Default)]
pub struct Command {
    pub name: String,
    pub summary: String,
    pub version: String,
    pub after_options: String,
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
        "md" => md::render(c),
        "fish" => fish::render(c),
        "zsh" => zsh::render(c),
        "man" => man::render(c),
        "sh" | "bash" | "csh" | "elvish" | "powershell" => panic!("shell '{shell}' completion is not implemented yet!"),
        _ => panic!("unknown option '{shell}'! Expected one of: \"md\", \"fish\", \"zsh\", \"man\", \"sh\", \"bash\", \"csh\", \"elvish\", \"powershell\""),
    }
}
