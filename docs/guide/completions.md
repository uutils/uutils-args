# Completions

Shell completions and documentation can be generated automatically by this crate. The implementation for this is in the [`uutils-args-complete`] crate. The easiest way of generating completions is via the `parse-is-complete` feature flag. This feature flag hijacks the [`Options::parse`](crate::Options::parse) function to print completions. This means that there is usually no need to write any additional code to generate completions.

```bash
cargo run --features parse-is-complete -- [shell]
```

The `[shell]` value here can be `fish`, `zsh`, `bash`, `powershell`, `elvish` or `nu`.

> **Note**: Some of these remain unimplemented as of writing.

Additionally, the values `man` or `md` can be passed to generate man pages and markdown documentation (for `mdbook`).

If you do not want to hijack the [`Options::parse`](crate::Options::parse) function, you can instead enable the `complete` feature flag. This makes the `Options::complete` function available in addition to the [`Options::parse`](crate::Options::parse) function to generate a `String` with the completion.
