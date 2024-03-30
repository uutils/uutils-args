// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Generation of completion and documentation
//!
//! All formats use the [`Command`] struct as input, which specifies all
//! information needed. This struct is similar to some structs in the derive
//! crate for uutils-args, but there are some key differences:
//!
//!  - This is meant to be more general.
//!  - Some information is added (such as fields for the summary)
//!  - We have [`ValueHint`] in this crate.
//!  - Some information is removed because it is irrelevant for completion and documentation
//!  - This struct is meant to exist at runtime of the program
//!
mod bash;
mod fish;
mod man;
mod md;
mod nu;
mod zsh;

/// A description of a CLI command
///
/// The completions and documentation will be generated based on this struct.
#[derive(Default)]
pub struct Command<'a> {
    pub name: &'a str,
    pub summary: &'a str,
    pub version: &'a str,
    pub after_options: &'a str,
    pub args: Vec<Arg<'a>>,
    pub license: &'a str,
    pub authors: &'a str,
}

/// Description of an argument
///
/// An argument may consist of several flags. In completions and documentation
/// formats that support it, these flags will be grouped.
#[derive(Default)]
pub struct Arg<'a> {
    pub short: Vec<Flag<'a>>,
    pub long: Vec<Flag<'a>>,
    pub help: &'a str,
    pub value: Option<ValueHint>,
}

pub struct Flag<'a> {
    pub flag: &'a str,
    pub value: Value<'a>,
}

pub enum Value<'a> {
    Required(&'a str),
    Optional(&'a str),
    No,
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
        "nu" | "nushell" => nu::render(c),
        "man" => man::render(c),
        "bash" => bash::render(c),
        "sh" | "csh" | "elvish" | "powershell" => panic!("shell '{shell}' completion is not implemented yet!"),
        _ => panic!("unknown option '{shell}'! Expected one of: \"md\", \"fish\", \"zsh\", \"nu[shell]\", \"man\", \"sh\", \"bash\", \"csh\", \"elvish\", \"powershell\""),
    }
}
