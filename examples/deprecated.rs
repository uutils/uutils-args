use uutils_args::{Arguments, Options};

fn parse_minus(s: &str) -> Option<&str> {
    let num = s.strip_prefix('-')?;
    if num.chars().all(|c| c.is_ascii_digit()) {
        Some(num)
    } else {
        None
    }
}
fn parse_plus(s: &str) -> Option<&str> {
    let num = s.strip_prefix('+')?;
    let num2 = num.strip_prefix('-').unwrap_or(num);
    if num2.chars().all(|c| c.is_ascii_digit()) {
        Some(num)
    } else {
        None
    }
}

#[derive(Arguments)]
enum Arg {
    #[arg(parse_minus)]
    Min(usize),

    #[arg(parse_plus)]
    Plus(isize),
}

#[derive(Default)]
struct Settings {
    n1: usize,
    n2: isize,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::Min(n) => self.n1 = n,
            Arg::Plus(n) => self.n2 = n,
        }
        Ok(())
    }
}

fn main() {
    assert_eq!(
        Settings::default().parse(["test", "-10"]).unwrap().0.n1,
        10usize
    );
    assert!(Settings::default().parse(["test", "--10"]).is_err());
    assert_eq!(
        Settings::default().parse(["test", "+10"]).unwrap().0.n2,
        10isize
    );
    assert_eq!(
        Settings::default().parse(["test", "+-10"]).unwrap().0.n2,
        -10isize
    );
}
