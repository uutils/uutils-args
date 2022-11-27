pub use derive::*;

pub trait Options {
    fn parse(args: &[&str]) -> Self;
}
