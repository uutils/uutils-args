#[test]
fn derive_error_messages_common() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/derive/arg_bare_keyword.rs");
    t.compile_fail("tests/derive/arg_just_minus.rs");
    t.compile_fail("tests/derive/arg_key_value.rs");
    t.compile_fail("tests/derive/arg_missing_closing_bracket.rs");
    t.compile_fail("tests/derive/arg_missing_equals.rs");
    t.compile_fail("tests/derive/arg_missing_field.rs");
    t.compile_fail("tests/derive/arg_missing_metavar.rs");
    t.compile_fail("tests/derive/arguments_file_nonexistent.rs");
    t.compile_fail("tests/derive/value_bare_keyword.rs");
    t.pass("tests/derive/arg_duplicate_other.rs"); // FIXME: Should fail!
    t.pass("tests/derive/arg_duplicate_within.rs"); // FIXME: Should fail!
}

#[cfg(unix)]
#[test]
fn derive_error_messages_unix() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/derive/arguments_file_isdir.rs"); // Needs the directory "/"
}

#[cfg(target_os = "linux")]
#[test]
fn derive_error_messages_linux_writeonly_file() {
    use std::fs::metadata;
    use std::os::unix::fs::PermissionsExt;

    // First, verify that /proc/self/clear_refs exists and is write-only:
    // https://man.archlinux.org/man/proc_pid_clear_refs.5.en
    let metadata = metadata("/proc/self/clear_refs").expect("should be in Linux 2.6.22");
    eprintln!("is_file={}", metadata.is_file());
    eprintln!("permissions={:?}", metadata.permissions());
    assert_eq!(0o100200, metadata.permissions().mode());

    // The file exists, as it should. Now we can run the test, using this
    // special write-only file, without having to worry about clean-up:
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/derive/arguments_file_writeonly.rs");
}
