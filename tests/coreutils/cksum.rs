use uutils_args::{Arguments, Options};

#[derive(Debug, Clone, Arguments)]
enum Arg {
    #[arg("-b", "--binary")]
    Binary,

    #[arg("-t", "--text")]
    Text,

    #[arg("--tag")]
    Tag,

    #[arg("--untagged")]
    Untagged,
}

#[derive(Default, Debug, PartialEq)]
enum Tristate {
    True,
    #[default]
    Unset,
    False,
}

#[derive(Default, Debug)]
struct Settings {
    binary: Tristate,
    tag: Tristate,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::Binary => self.binary = Tristate::True,
            Arg::Text => self.binary = Tristate::False,
            Arg::Tag => {
                // https://github.com/uutils/coreutils/issues/6364
                self.binary = Tristate::Unset;
                self.tag = Tristate::True;
            }
            Arg::Untagged => {
                // https://github.com/uutils/coreutils/issues/6364
                if self.tag == Tristate::True {
                    self.binary = Tristate::Unset;
                }
                self.tag = Tristate::False;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum ResultingFormat {
    UntaggedText,
    UntaggedBinary,
    Tagged,
    ErrorInstead,
}

impl Settings {
    fn format(&self) -> ResultingFormat {
        // Interpret "Unset" as "tagged":
        if self.tag != Tristate::False {
            // -> Tagged.
            // Error only if the user explicitly requests the text format:
            if self.binary == Tristate::False {
                ResultingFormat::ErrorInstead
            } else {
                ResultingFormat::Tagged
            }
        } else {
            // -> Untagged.
            // Binary only if the user explicitly requests it:
            if self.binary == Tristate::True {
                ResultingFormat::UntaggedBinary
            } else {
                ResultingFormat::UntaggedText
            }
        }
    }
}

// Convenience function for testing
#[cfg(test)]
fn assert_format(args: &[&str], expected: ResultingFormat) {
    let mut full_argv = vec!["bin_name"];
    full_argv.extend(args);
    let result = Settings::default().parse(full_argv).unwrap();
    assert_eq!(
        (result.0.format(), result.1.as_slice()),
        (expected, [].as_slice()),
        "{args:?}"
    );
}

// These tests basically force the reader to make the same conclusions as
// https://github.com/uutils/coreutils/issues/6364
// Quotes from the issue are marked with a leading ">".

#[test]
fn binary_text_toggle_in_tagged() {
    // > Observe that -b/-t seems to be doing precisely what we would hope for: toggle between binary/text mode:
    // -b/-t/--tagged switch between tagged/error behavior
    assert_format(&[], ResultingFormat::Tagged);
    assert_format(&["-t"], ResultingFormat::ErrorInstead);
    assert_format(&["-t", "-b"], ResultingFormat::Tagged);
    assert_format(&["-t", "--tag"], ResultingFormat::Tagged);
}

#[test]
fn binary_text_toggle_in_untagged() {
    // Once we're in untagged format, -b/-t switch between binary/text behavior
    assert_format(&["--untagged"], ResultingFormat::UntaggedText);
    assert_format(&["--untagged", "-t"], ResultingFormat::UntaggedText);
    assert_format(&["--untagged", "-b"], ResultingFormat::UntaggedBinary);
    assert_format(&["--untagged", "-t", "-b"], ResultingFormat::UntaggedBinary);
    assert_format(&["--untagged", "-b", "-t"], ResultingFormat::UntaggedText);
}

// > Observe that --tag/--untagged seems to be the flags that have the weird behavior attached to
// > them. In particular, the T state seems to be more that one actual state, probably
// > differentiated along the "text-binary-axis".

#[test]
fn nondeterministic_edges() {
    // Same behavior:
    assert_format(&[], ResultingFormat::Tagged);
    assert_format(&["-b"], ResultingFormat::Tagged);
    // But must have different internal state:
    assert_format(&["--untagged"], ResultingFormat::UntaggedText);
    assert_format(&["-b", "--untagged"], ResultingFormat::UntaggedBinary);
}

#[test]
fn selfloops() {
    // "T"
    assert_format(&[], ResultingFormat::Tagged);
    assert_format(&["-b"], ResultingFormat::Tagged);
    assert_format(&["--tag"], ResultingFormat::Tagged);
    assert_format(&["-b", "--tag"], ResultingFormat::Tagged);
    // "E"
    assert_format(&["-t"], ResultingFormat::ErrorInstead);
    assert_format(&["-t", "-t"], ResultingFormat::ErrorInstead);
    // "A"
    assert_format(&["-b", "--untagged"], ResultingFormat::UntaggedBinary);
    assert_format(&["-b", "--untagged", "-b"], ResultingFormat::UntaggedBinary);
    assert_format(
        &["-b", "--untagged", "--untagged"],
        ResultingFormat::UntaggedBinary,
    );
    // "S"
    assert_format(&["--untagged"], ResultingFormat::UntaggedText);
    assert_format(&["--untagged", "-t"], ResultingFormat::UntaggedText);
    assert_format(&["--untagged", "--untagged"], ResultingFormat::UntaggedText);
}

#[test]
fn other_diagonals() {
    // From "A" and "S" ...
    assert_format(&["-b", "--untagged"], ResultingFormat::UntaggedBinary);
    assert_format(&["--untagged"], ResultingFormat::UntaggedText);
    // ... to "T":
    assert_format(&["-b", "--untagged", "--tag"], ResultingFormat::Tagged);
    assert_format(&["--untagged", "--tag"], ResultingFormat::Tagged);
    // From "E" to "S":
    assert_format(&["-t"], ResultingFormat::ErrorInstead);
    assert_format(&["-t", "--untagged"], ResultingFormat::UntaggedText);
}

#[test]
fn suffix_b_u_not_deterministic() {
    // > Ending in bU does not determine the result:
    assert_format(&["-b", "--untagged"], ResultingFormat::UntaggedBinary);
    assert_format(
        &["--tag", "-b", "--untagged"],
        ResultingFormat::UntaggedText,
    );
    assert_format(
        &["--untagged", "-b", "--untagged"],
        ResultingFormat::UntaggedBinary,
    );
    assert_format(
        &["-b", "--untagged", "-b", "--untagged"],
        ResultingFormat::UntaggedBinary,
    );
    assert_format(
        &["--tag", "--untagged", "-b", "--untagged"],
        ResultingFormat::UntaggedBinary,
    );
    assert_format(
        &["--untagged", "--tag", "-b", "--untagged"],
        ResultingFormat::UntaggedText,
    );
    // > Therefore, U does not set the binary-ness to a constant, but rather depends on the tagged-ness.
}

// I *think* that this battery of tests fully specifies the full behavior.
// In any case, brute-forcing all of the 4^n combinations up to 5 arguments
// shows no counter-examples, so this implementation is definitely a good match.
