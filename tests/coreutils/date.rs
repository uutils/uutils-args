use std::ffi::OsString;
use uutils_args::{Arguments, Options, Value};

// Note: "+%s"-style format options aren't covered here, but should be!

// +%s
//   -I[FMT], --iso-8601[=FMT]  output date/time in ISO 8601 format.
//   -R, --rfc-email            output date and time in RFC 5322 format.
//       --rfc-3339=FMT         output date/time in RFC 3339 format.
// date, hours, minutes, seconds, ns
// date, seconds, ns

#[derive(Default, Debug, PartialEq, Eq, Value)]
enum Iso8601Format {
    #[default]
    #[value("date")]
    // TODO: Express the concept "accepts prefixes" more nicely.
    #[value("dat")]
    #[value("da")]
    #[value("d")]
    Date,

    #[value("hours")]
    // TODO: Express the concept "accepts prefixes" more nicely.
    #[value("hour")]
    #[value("hou")]
    #[value("ho")]
    #[value("h")]
    Hours,

    #[value("minutes")]
    // TODO: Express the concept "accepts prefixes" more nicely.
    #[value("minute")]
    #[value("minut")]
    #[value("minu")]
    #[value("min")]
    #[value("mi")]
    #[value("m")]
    Minutes,

    #[value("seconds")]
    // TODO: Express the concept "accepts prefixes" more nicely.
    #[value("second")]
    #[value("secon")]
    #[value("seco")]
    #[value("sec")]
    #[value("se")]
    #[value("s")]
    Seconds,

    #[value("ns")]
    // TODO: Express the concept "accepts prefixes" more nicely.
    #[value("n")]
    Ns,
}

#[derive(Debug, PartialEq, Eq, Value)]
enum Rfc3339Format {
    #[value("date")]
    // TODO: Express the concept "accepts prefixes" more nicely.
    #[value("dat")]
    #[value("da")]
    #[value("d")]
    Date,

    #[value("seconds")]
    // TODO: Express the concept "accepts prefixes" more nicely.
    #[value("second")]
    #[value("secon")]
    #[value("seco")]
    #[value("sec")]
    #[value("se")]
    #[value("s")]
    Seconds,

    #[value("ns")]
    // TODO: Express the concept "accepts prefixes" more nicely.
    #[value("n")]
    Ns,
}

#[derive(Arguments)]
enum Arg {
    #[arg("-I[FMT]")]
    #[arg("--iso-8601[=FMT]")]
    Iso(Iso8601Format),

    #[arg("--rfc-3339=FMT")]
    Rfc3339(Rfc3339Format),

    #[arg("-R")]
    #[arg("--rfc-email")]
    RfcEmail,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum Format {
    #[default]
    Unspecified,
    Iso8601(Iso8601Format),
    Rfc3339(Rfc3339Format),
    RfcEmail,
    // FromString(OsString),
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Settings {
    chosen_format: Format,
}

const MAGIC_MULTI_OUTPUT_ARG: &str = "! multiformat";

impl Options<Arg> for Settings {
    fn apply(&mut self, arg: Arg) -> Result<(), uutils_args::Error> {
        if self.chosen_format != Format::Unspecified {
            return Err(uutils_args::Error {
                exit_code: 1,
                kind: uutils_args::ErrorKind::UnexpectedArgument(MAGIC_MULTI_OUTPUT_ARG.to_owned()),
            });
        }
        match arg {
            Arg::Iso(iso) => self.chosen_format = Format::Iso8601(iso),
            Arg::Rfc3339(rfc3339) => self.chosen_format = Format::Rfc3339(rfc3339),
            Arg::RfcEmail => self.chosen_format = Format::RfcEmail,
        }
        Ok(())
    }
}

#[test]
fn noarg() {
    let (settings, operands) = Settings::default().parse(["date"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Unspecified);
}

#[test]
fn iso_short_noarg() {
    let (settings, operands) = Settings::default().parse(["date", "-I"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_short_arg_direct_date() {
    let (settings, operands) = Settings::default().parse(["date", "-Idate"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_short_arg_equal_date() {
    // Not accepted by GNU, but we want to accept it.
    let (settings, operands) = Settings::default().parse(["date", "-I=date"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_short_arg_space_date() {
    let (settings, operands) = Settings::default().parse(["date", "-I", "date"]).unwrap();
    // Must not be interpreted as an argument to "-I".
    assert_eq!(operands, vec!["date"]);
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_short_arg_direct_minutes() {
    let (settings, operands) = Settings::default().parse(["date", "-Iminutes"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Minutes)
    );
}

#[test]
fn iso_short_arg_equal_minutes() {
    // Not accepted by GNU, but we want to accept it.
    let (settings, operands) = Settings::default().parse(["date", "-I=minutes"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Minutes)
    );
}

#[test]
fn iso_short_arg_space_minutes() {
    let (settings, operands) = Settings::default()
        .parse(["date", "-I", "minutes"])
        .unwrap();
    // Must not be interpreted as an argument to "-I".
    assert_eq!(operands, vec!["minutes"]);
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_short_arg_invalid() {
    let the_err = Settings::default()
        .parse(["date", "-Idefinitely_invalid"])
        .unwrap_err();
    // Must not be interpreted as an argument to "-I".
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::ParsingFailed { option, value, .. } => {
            assert_eq!(option, "-I");
            assert_eq!(value, "definitely_invalid");
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn iso_short_arg_equal_hours() {
    let (settings, operands) = Settings::default().parse(["date", "-I=hours"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Hours)
    );
}

#[test]
fn iso_short_arg_equal_seconds() {
    let (settings, operands) = Settings::default().parse(["date", "-I=seconds"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Seconds)
    );
}

#[test]
fn iso_short_arg_equal_ns() {
    let (settings, operands) = Settings::default().parse(["date", "-I=ns"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Ns));
}

#[test]
fn iso_short_arg_equal_hour_singular() {
    let (settings, operands) = Settings::default().parse(["date", "-I=hour"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Hours)
    );
}

#[test]
fn iso_short_arg_equal_second_singular() {
    let (settings, operands) = Settings::default().parse(["date", "-I=second"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Seconds)
    );
}

#[test]
fn iso_short_arg_equal_minute_singular() {
    let (settings, operands) = Settings::default().parse(["date", "-I=minute"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Minutes)
    );
}

#[test]
fn iso_short_arg_equal_n_singular() {
    let (settings, operands) = Settings::default().parse(["date", "-I=n"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Ns));
}

#[test]
fn iso_long_noarg() {
    let (settings, operands) = Settings::default().parse(["date", "--iso-8601"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_long_equal_date() {
    let (settings, operands) = Settings::default()
        .parse(["date", "--iso-8601=date"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_long_equal_hour() {
    let (settings, operands) = Settings::default()
        .parse(["date", "--iso-8601=hour"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Hours)
    );
}

#[test]
fn iso_long_space_hour() {
    let (settings, operands) = Settings::default()
        .parse(["date", "--iso-8601", "hour"])
        .unwrap();
    // Must not be interpreted as an argument to "-I".
    assert_eq!(operands, vec!["hour"]);
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_long_equal_n() {
    let (settings, operands) = Settings::default().parse(["date", "--iso-8601=n"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Ns));
}

#[test]
fn rfc3339_noarg() {
    let the_err = Settings::default()
        .parse(["date", "--rfc-3339"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::MissingValue { option } => {
            assert_eq!(option, Some("--rfc-3339".to_owned()));
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc3339_equal_date() {
    let (settings, operands) = Settings::default()
        .parse(["date", "--rfc-3339=date"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Rfc3339(Rfc3339Format::Date));
}

#[test]
fn rfc3339_equal_ns() {
    let (settings, operands) = Settings::default()
        .parse(["date", "--rfc-3339=ns"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Rfc3339(Rfc3339Format::Ns));
}

#[test]
fn rfc3339_equal_n_singular() {
    let (settings, operands) = Settings::default().parse(["date", "--rfc-3339=n"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Rfc3339(Rfc3339Format::Ns));
}

#[test]
fn rfc3339_equal_minutes() {
    let the_err = Settings::default()
        .parse(["date", "--rfc-3339=minutes"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::ParsingFailed { option, value, .. } => {
            assert_eq!(option, "--rfc-3339");
            assert_eq!(value, "minutes");
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc3339_space_date() {
    let (settings, operands) = Settings::default()
        .parse(["date", "--rfc-3339", "date"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Rfc3339(Rfc3339Format::Date));
}

#[test]
fn rfc3339_space_ns() {
    let (settings, operands) = Settings::default()
        .parse(["date", "--rfc-3339", "ns"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Rfc3339(Rfc3339Format::Ns));
}

#[test]
fn rfc3339_space_n_singular() {
    let (settings, operands) = Settings::default()
        .parse(["date", "--rfc-3339", "n"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Rfc3339(Rfc3339Format::Ns));
}

#[test]
fn rfc3339_space_minutes() {
    let the_err = Settings::default()
        .parse(["date", "--rfc-3339", "minutes"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::ParsingFailed { option, value, .. } => {
            assert_eq!(option, "--rfc-3339");
            assert_eq!(value, "minutes");
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_email_short() {
    let (settings, operands) = Settings::default().parse(["date", "-R"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::RfcEmail);
}

#[test]
fn rfc_email_long() {
    let (settings, operands) = Settings::default().parse(["date", "--rfc-email"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::RfcEmail);
}

#[test]
fn rfc_clash_isoshort_isoshort() {
    let the_err = Settings::default().parse(["date", "-I", "-I"]).unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_isoshort_isolong() {
    let the_err = Settings::default()
        .parse(["date", "-I", "--iso-8601"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_isoshort_rfc3339() {
    let the_err = Settings::default()
        .parse(["date", "-I", "--rfc-3339=date"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_isoshort_rfcemailshort() {
    let the_err = Settings::default().parse(["date", "-I", "-R"]).unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_isoshort_rfcemaillong() {
    let the_err = Settings::default()
        .parse(["date", "-I", "--rfc-email"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_isolong_isoshort() {
    let the_err = Settings::default()
        .parse(["date", "--iso-8601", "-I"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_isolong_isolong() {
    let the_err = Settings::default()
        .parse(["date", "--iso-8601", "--iso-8601"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_isolong_rfc3339() {
    let the_err = Settings::default()
        .parse(["date", "--iso-8601", "--rfc-3339=date"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_isolong_rfcemailshort() {
    let the_err = Settings::default()
        .parse(["date", "--iso-8601", "-R"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_isolong_rfcemaillong() {
    let the_err = Settings::default()
        .parse(["date", "--iso-8601", "--rfc-email"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_rfcemailshort_isoshort() {
    let the_err = Settings::default().parse(["date", "-R", "-I"]).unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_rfcemailshort_isolong() {
    let the_err = Settings::default()
        .parse(["date", "-R", "--iso-8601"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_rfcemailshort_rfc3339() {
    let the_err = Settings::default()
        .parse(["date", "-R", "--rfc-3339=date"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_rfcemailshort_rfcemailshort() {
    let the_err = Settings::default().parse(["date", "-R", "-R"]).unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
fn rfc_clash_rfcemailshort_rfcemaillong() {
    let the_err = Settings::default()
        .parse(["date", "-R", "--rfc-email"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

#[test]
#[ignore = "exits too early, but works correctly"]
fn default_show_help() {
    let (settings, operands) = Settings::default().parse(&["date", "--help"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Unspecified);
}

#[test]
#[ignore = "BROKEN, exits too early"]
fn rfcemail_show_help() {
    let (settings, operands) = Settings::default()
        .parse(&["date", "-R", "--help"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::RfcEmail);
}

#[test]
fn multi_output_has_priority() {
    let the_err = Settings::default()
        .parse(&["date", "-R", "-R", "--help"])
        .unwrap_err();
    assert_eq!(the_err.exit_code, 1);
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}

/// https://github.com/uutils/coreutils/issues/4254#issuecomment-2026446634
#[test]
fn priority_demo() {
    // Earliest faulty argument is the first argument, must complaint about that:
    let the_err = Settings::default()
        .parse(&["date", "-Idefinitely_invalid", "-R", "-R"])
        .unwrap_err();
    match the_err.kind {
        uutils_args::ErrorKind::ParsingFailed { option, value, .. } => {
            assert_eq!(option, "-I");
            assert_eq!(value, "definitely_invalid");
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
    // Earliest faulty argument is the second argument, must complaint about that:
    let the_err = Settings::default()
        .parse(&["date", "-R", "-R", "-Idefinitely_invalid"])
        .unwrap_err();
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
    // Earliest faulty argument is the second argument, must complaint about that:
    let the_err = Settings::default()
        .parse(&["date", "-R", "-Idefinitely_invalid", "-R"])
        .unwrap_err();
    match the_err.kind {
        uutils_args::ErrorKind::ParsingFailed { option, value, .. } => {
            assert_eq!(option, "-I");
            assert_eq!(value, "definitely_invalid");
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
    // Earliest faulty argument is the second argument, must complaint about that:
    let the_err = Settings::default()
        .parse(&["date", "-R", "-Ins", "-R"])
        .unwrap_err();
    match the_err.kind {
        uutils_args::ErrorKind::UnexpectedArgument(arg) => {
            assert_eq!(arg, MAGIC_MULTI_OUTPUT_ARG);
        }
        _ => panic!("wrong error kind: {:?}", the_err.kind),
    }
}
