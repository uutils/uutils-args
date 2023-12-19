<style>
.chapters p {
    display: grid;
    grid-template-columns: repeat(3, 6em);
    justify-content: space-between;
}
.chapters a {
    text-align: center;
    font-family: "Fira Sans",Arial,NanumBarunGothic,sans-serif;
    border: 1px solid var(--link-color);
    border-radius: 4px;
    padding: 3px 10px;
}
.chapters a[href=""] {
    pointer-events: none;
    color: var(--scrollbar-thumb-background-color);
    border: 1px solid var(--scrollbar-thumb-background-color);
}
</style>
<div class="chapters">

[Previous](previous)
[Up](super)
[Next](next)

</div>

# Value trait

Any field on the enum implementing [`Arguments`](trait@crate::Arguments) has to implement the [`Value`](trait@crate::Value) trait, which determines how it is derive from the text value. Normally, [`Value`](trait@crate::Value) only requires one method: [`from_value`](crate::Value::from_value), which takes an `&OsStr` and returns a `Result` with either `Self` or some boxed error.

This trait is implemented for common types, such as integers, [`OsString`](std::ffi::OsString), [`PathBuf`](std::path::PathBuf), [`String`] and [`Option<T>`] where `T` implements `Value`.

There is also a [`Value` derive macro](derive@crate::Value), which provides parsing string values into an `enum`. The name of each variant (lowercased) with a `#[value]` attribute is parsed automatically. Additionally, if the string is an unambiguous prefix, it is also parsed. For example, if we have the values `"yes"` and `"no"` then `"y"`, `"ye"`, `"yes"` are all valid for `"yes"`, because no other values start with those substrings.

```rust
use uutils_args::Value;
use std::ffi::OsStr;

#[derive(Value, Debug, PartialEq, Eq)]
enum YesOrNo {
    #[value]
    Yes,
    #[value]
    No,
}

assert_eq!(YesOrNo::from_value(OsStr::new("yes")).unwrap(), YesOrNo::Yes);
assert_eq!(YesOrNo::from_value(OsStr::new("no")).unwrap(), YesOrNo::No);
assert_eq!(YesOrNo::from_value(OsStr::new("y")).unwrap(), YesOrNo::Yes);
assert_eq!(YesOrNo::from_value(OsStr::new("n")).unwrap(), YesOrNo::No);
assert!(YesOrNo::from_value(OsStr::new("YES")).is_err());
assert!(YesOrNo::from_value(OsStr::new("NO")).is_err());
assert!(YesOrNo::from_value(OsStr::new("maybe")).is_err());
```

We can also provide custom names for the variants. This is useful if there are multiple strings that should parse to one variant.

```rust
use uutils_args::Value;
use std::ffi::OsStr;

#[derive(Value, Debug, PartialEq, Eq)]
enum Color {
    #[value("yes", "always")]
    Always,
    #[value("auto")]
    Auto,
    #[value("no", "never")]
    Never,
}

assert_eq!(Color::from_value(&OsStr::new("yes")).unwrap(), Color::Always);
assert_eq!(Color::from_value(&OsStr::new("always")).unwrap(), Color::Always);
assert_eq!(Color::from_value(&OsStr::new("auto")).unwrap(), Color::Auto);
assert_eq!(Color::from_value(&OsStr::new("no")).unwrap(), Color::Never);
assert_eq!(Color::from_value(&OsStr::new("never")).unwrap(), Color::Never);

// The prefixes here are interesting:
// - "a" is ambiguous because it is a prefix of "auto" and "always"
// - "n" is not ambiguous because "no" and "never" map to the same variant
assert!(Color::from_value(&OsStr::new("a")).is_err());
assert_eq!(Color::from_value(&OsStr::new("n")).unwrap(), Color::Never);
```

<div class="chapters">

[Previous](previous)
[Up](super)
[Next](next)

</div>