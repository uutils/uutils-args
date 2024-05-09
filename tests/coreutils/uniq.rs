use uutils_args::{Arguments, Initial, Options, Value};

// TODO: Deprecated syntax
#[derive(Arguments)]
enum Arg {
    #[option("-f N", "--skip-fields=n")]
    SkipFields(usize),

    #[option("-s N", "--skip-chars=N")]
    SkipChars(usize),

    #[option("-c", "--count")]
    Count,

    #[option("-i", "--ignore-case")]
    IgnoreCase,

    #[option("-d", "--repeated")]
    Repeated,

    #[option("-D", "--all-repeated[=delimit-method]")]
    AllRepeated(Delimiters),

    #[option("--group[=delimit-method]", default=Delimiters::Separate)]
    Group(Delimiters),

    #[option("-u", "--unique")]
    Unique,

    #[option("-w N", "--check-chars=N")]
    CheckChars(usize),

    #[option("-z", "--zero-terminated")]
    ZeroTerminated,
}

#[derive(Value, Default)]
enum Delimiters {
    #[default]
    #[value("none")]
    None,
    #[value("prepend")]
    Prepend,
    #[value("append")]
    Append,
    #[value("separate")]
    Separate,
    // Note: both is not an accepted argument of -D/--all-repeated
    #[value("both")]
    Both,
}

#[derive(Default)]
struct Settings {
    repeats_only: bool,
    uniques_only: bool,
    all_repeated: bool,
    delimiters: Delimiters,
    show_counts: bool,
    skip_fields: Option<usize>,
    slice_start: Option<usize>,
    slice_stop: Option<usize>,
    ignore_case: bool,
    zero_terminated: bool,
}

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        match arg {
            Arg::SkipFields(n) => {
                self.skip_fields = Some(n);
            }
            Arg::SkipChars(n) => {
                self.slice_start = Some(n);
            }
            Arg::Count => {
                self.show_counts = true;
            }
            Arg::IgnoreCase => {
                self.ignore_case = true;
            }
            Arg::Repeated => {
                self.repeats_only = true;
            }
            Arg::AllRepeated(d) => {
                self.repeats_only = true;
                self.all_repeated = true;
                self.delimiters = d;
            }
            Arg::Group(d) => {
                self.all_repeated = true;
                self.delimiters = d;
            }
            Arg::Unique => {
                self.uniques_only = true;
            }
            Arg::CheckChars(n) => {
                self.slice_stop = Some(n);
            }
            Arg::ZeroTerminated => {
                self.zero_terminated = true;
            }
        };
        Ok(())
    }
}
