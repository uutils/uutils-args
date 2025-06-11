use std::ffi::OsString;
use uutils_args::{Arguments, Options, Parsed, Value};

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
    Date,

    #[value("hours")]
    Hours,

    #[value("minutes")]
    Minutes,

    #[value("seconds")]
    Seconds,

    #[value("ns")]
    Ns,
}

#[derive(Debug, PartialEq, Eq, Value)]
enum Rfc3339Format {
    #[value("date")]
    Date,

    #[value("seconds")]
    Seconds,

    #[value("ns")]
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
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Unspecified);
}

#[test]
fn iso_short_noarg() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-I"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_short_arg_direct_date() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-Idate"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_short_arg_equal_date() {
    // Not accepted by GNU, but we want to accept it.
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-I=date"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_short_arg_space_date() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-I", "date"]).unwrap();
    // Must not be interpreted as an argument to "-I".
    assert_eq!(operands, vec!["date"]);
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_short_arg_direct_minutes() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-Iminutes"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Minutes)
    );
}

#[test]
fn iso_short_arg_equal_minutes() {
    // Not accepted by GNU, but we want to accept it.
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-I=minutes"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Minutes)
    );
}

#[test]
fn iso_short_arg_space_minutes() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default()
        .parse(&["date", "-I", "minutes"])
        .unwrap();
    // Must not be interpreted as an argument to "-I".
    assert_eq!(operands, vec!["minutes"]);
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_short_arg_invalid() {
    // Must not be interpreted as an argument to "-I".
    match Settings::default().parse(&["date", "-Idefinitely_invalid"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::ParsingFailed { option, value, .. }
                    if *option == "-I" && value == "definitely_invalid"
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn iso_short_arg_equal_hours() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-I=hours"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Hours)
    );
}

#[test]
fn iso_short_arg_equal_seconds() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-I=seconds"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Seconds)
    );
}

#[test]
fn iso_short_arg_equal_ns() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-I=ns"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Ns));
}

#[test]
fn iso_short_arg_equal_hour_singular() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-I=hour"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Hours)
    );
}

#[test]
fn iso_short_arg_equal_second_singular() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-I=second"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Seconds)
    );
}

#[test]
fn iso_short_arg_equal_minute_singular() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-I=minute"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Minutes)
    );
}

#[test]
fn iso_short_arg_equal_n_singular() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-I=n"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Ns));
}

#[test]
fn iso_long_noarg() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "--iso-8601"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_long_equal_date() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default()
        .parse(&["date", "--iso-8601=date"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_long_equal_hour() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default()
        .parse(&["date", "--iso-8601=hour"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(
        settings.chosen_format,
        Format::Iso8601(Iso8601Format::Hours)
    );
}

#[test]
fn iso_long_space_hour() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default()
        .parse(&["date", "--iso-8601", "hour"])
        .unwrap();
    // Must not be interpreted as an argument to "-I".
    assert_eq!(operands, vec!["hour"]);
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Date));
}

#[test]
fn iso_long_equal_n() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default()
        .parse(&["date", "--iso-8601=n"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Iso8601(Iso8601Format::Ns));
}

#[test]
fn rfc3339_noarg() {
    match Settings::default().parse(&["date", "--rfc-3339"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::MissingValue { option }
                    if *option == Some("--rfc-3339".to_string())
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc3339_equal_date() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default()
        .parse(&["date", "--rfc-3339=date"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Rfc3339(Rfc3339Format::Date));
}

#[test]
fn rfc3339_equal_ns() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default()
        .parse(&["date", "--rfc-3339=ns"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Rfc3339(Rfc3339Format::Ns));
}

#[test]
fn rfc3339_equal_n_singular() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default()
        .parse(&["date", "--rfc-3339=n"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Rfc3339(Rfc3339Format::Ns));
}

#[test]
fn rfc3339_equal_minutes() {
    match Settings::default().parse(&["date", "--rfc-3339=minutes"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::ParsingFailed { option, value, .. }
                    if *option == "--rfc-3339" && value == "minutes"
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc3339_space_date() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default()
        .parse(&["date", "--rfc-3339", "date"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Rfc3339(Rfc3339Format::Date));
}

#[test]
fn rfc3339_space_ns() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default()
        .parse(&["date", "--rfc-3339", "ns"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Rfc3339(Rfc3339Format::Ns));
}

#[test]
fn rfc3339_space_n_singular() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default()
        .parse(&["date", "--rfc-3339", "n"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Rfc3339(Rfc3339Format::Ns));
}

#[test]
fn rfc3339_space_minutes() {
    match Settings::default().parse(&["date", "--rfc-3339", "minutes"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::ParsingFailed { option, value, .. }
                    if *option == "--rfc-3339" && value == "minutes"
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_email_short() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "-R"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::RfcEmail);
}

#[test]
fn rfc_email_long() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "--rfc-email"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::RfcEmail);
}

#[test]
fn rfc_clash_isoshort_isoshort() {
    match Settings::default().parse(&["date", "-I", "-I"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_isoshort_isolong() {
    match Settings::default().parse(&["date", "-I", "--iso-8601"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_isoshort_rfc3339() {
    match Settings::default().parse(&["date", "-I", "--rfc-3339=date"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_isoshort_rfcemailshort() {
    match Settings::default().parse(&["date", "-I", "-R"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_isoshort_rfcemaillong() {
    match Settings::default().parse(&["date", "-I", "--rfc-email"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_isolong_isoshort() {
    match Settings::default().parse(&["date", "--iso-8601", "-I"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_isolong_isolong() {
    match Settings::default().parse(&["date", "--iso-8601", "--iso-8601"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_isolong_rfc3339() {
    match Settings::default().parse(&["date", "--iso-8601", "--rfc-3339=date"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_isolong_rfcemailshort() {
    match Settings::default().parse(&["date", "--iso-8601", "-R"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_isolong_rfcemaillong() {
    match Settings::default().parse(&["date", "--iso-8601", "--rfc-email"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_rfcemailshort_isoshort() {
    match Settings::default().parse(&["date", "-R", "-I"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_rfcemailshort_isolong() {
    match Settings::default().parse(&["date", "-R", "--iso-8601"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_rfcemailshort_rfc3339() {
    match Settings::default().parse(&["date", "-R", "--rfc-3339=date"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_rfcemailshort_rfcemailshort() {
    match Settings::default().parse(&["date", "-R", "-R"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
fn rfc_clash_rfcemailshort_rfcemaillong() {
    match Settings::default().parse(&["date", "-R", "--rfc-email"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

#[test]
#[ignore = "exits too early, but works correctly"]
fn default_show_help() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default().parse(&["date", "--help"]).unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::Unspecified);
}

#[test]
#[ignore = "BROKEN, exits too early"]
fn rfcemail_show_help() {
    let Parsed::<Settings> {
        settings, operands, ..
    } = Settings::default()
        .parse(&["date", "-R", "--help"])
        .unwrap();
    assert_eq!(operands, Vec::<OsString>::new());
    assert_eq!(settings.chosen_format, Format::RfcEmail);
}

#[test]
fn multi_output_has_priority() {
    match Settings::default().parse(&["date", "-R", "-R", "--help"]) {
        Err(err)
            if err.exit_code == 1
                && matches!(
                    &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                    if arg == MAGIC_MULTI_OUTPUT_ARG
                ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}

/// https://github.com/uutils/coreutils/issues/4254#issuecomment-2026446634
#[test]
fn priority_demo() {
    // Earliest faulty argument is the first argument, must complaint about that:
    match Settings::default().parse(&["date", "-Idefinitely_invalid", "-R", "-R"]) {
        Err(err)
            if matches!(
                &err.kind, uutils_args::ErrorKind::ParsingFailed { option, value, .. }
                if *option == "-I" && value == "definitely_invalid"
            ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
    // Earliest faulty argument is the second argument, must complaint about that:
    match Settings::default().parse(&["date", "-R", "-R", "-Idefinitely_invalid"]) {
        Err(err)
            if matches!(
                &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                if arg == MAGIC_MULTI_OUTPUT_ARG
            ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
    // Earliest faulty argument is the second argument, must complaint about that:
    match Settings::default().parse(&["date", "-R", "-Idefinitely_invalid", "-R"]) {
        Err(err)
            if matches!(
                &err.kind, uutils_args::ErrorKind::ParsingFailed { option, value, .. }
                if *option == "-I" && value == "definitely_invalid"
            ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
    // Earliest faulty argument is the second argument, must complaint about that:
    match Settings::default().parse(&["date", "-R", "-Ins", "-R"]) {
        Err(err)
            if matches!(
                &err.kind, uutils_args::ErrorKind::UnexpectedArgument(arg)
                if arg == MAGIC_MULTI_OUTPUT_ARG
            ) => {}
        Err(other_err) => panic!("wrong error kind: {:?}", other_err.kind),
        _ => panic!("should not be parsed"),
    }
}
