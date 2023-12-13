# Library design

In this document, I explain how this library solves the problems with `clap` and
how it accomplishes the design goals.

## Basic API

This library only has a derive API. In most derive-based argument parsers, the
arguments are based on a `struct`, but in this library they are based on `enum`
variants, which then get mapped to a `struct`. The parsing happens in two stages

1. Arguments get mapped to an `enum`
2. The `enum` variants are matched and update `struct` fields.

This gives us a separation of concerns: the `enum` determines how the arguments
get parsed and the `struct` determines how they map to the program settings.
This gives us a lot of freedom in defining our mapping from arguments to
settings.

Here is a simple example comparing `clap` and `uutils_args`.

> **Note**: There are differences in behaviour between these two. E.g.
> uutils_args allows options to appear multiple times, remembering only the last
> one.

```rust
// Clap
#[derive(Parser)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long)]
    say_goodbye: bool,
}

// Uutils args
#[derive(Arguments, Clone)]
enum Arg {
    /// Name of the person to greet
    #[option("-n NAME", "--name=NAME")]
    Name(String),

    /// Number of times to greet
    #[option("-g", "--goodbye")]
    SayGoodbye
}

#[derive(Options, Default)]
#[arg_type(Arg)]
struct Settings {
    #[set(Arg::Name)]
    name: String

    #[map(Arg::SayGoodbye => true)]
    goodbye: bool,
}
```

> **Note**: `uutils_args` is more explicit than `clap`, you have to explicitly
> state the names of the flags and values. This helps maintainability because it
> is always obvious where an argument is defined.

As part of the `Options` derive, we get a `Settings::parse` method that returns
a `Settings` from a `OsString` iterator. The implementation of this is defined
by the `set` and `map` attributes. `map` just says: "if we encounter this value
in the iterator set this value", using a match-like syntax (it expands to a
match). And the `#[set(Arg::Name)]` is just short for
`#[map(Arg::Name(name) => name)]`, because that is a commonly appearing pattern.

Importantly, arguments can appear in the attributes for multiple fields. We
could for instance do this:

```rust
#[derive(Arguments, Clone)]
enum Arg {
    #[option("-a")]
    A,

    #[option("--a-and-b")]
    B
}

#[derive(Options, Default)]
#[arg_type(Arg)]
struct Settings {
    #[map(Arg::A | Arg::B => true)]
    a: bool

    #[map(Arg::B => true)]
    b: bool,
}
```

## Argument types

```rust
#[derive(Arguments, Clone)]
#[help("--help")] // help and version must be explicitly defined
#[version("--version")]
enum Arg {
    // Note: You can have as many flags as you want for each variable
    #[option("-f", "--foo")]
    Flag,

    // Note: The value name is required and will be used in `--help`
    #[option("-r VALUE", "--required=VALUE")]
    OptionWithRequiredValue(String),

    // Note: The value name is again required.
    // Note: If no `default` is specified, `Default::default` is used.
    #[option("-o[VALUE]", "--optional[=VALUE]", default = "DEFAULT".into())]
    OptionWithOptionalValue(String),

    // Note: `-l` will use the default value.
    #[option("-l", "--long=VALUE", default = "SHORT VALUE")]
    ValueOnlyForLongOption(String),

    // Any combination of required, optional and no arguments is possible.
    #[option("-t VAL", "--test[=VAL]", default = "")]
    ValueOptionalForLongOption(String),

    // Positional arguments take a range of the number of arguments they
    // take. The default is 1..=1, i.e. exactly 1 argument.
    #[positional]
    SinglePositionalArgument(String),

    #[positional(0..=1)]
    OptionalPositionalArgument(String),

    // Range is open on both sides so 0..=MAX
    #[positional(..)]
    AnyNumberOfPositionalArguments(String),

    // All remaining arguments are collected into a `Vec`.
    #[position(last)]
    TrailingVarArg(Vec<String>),

    // Same range can still be applied even though there can only ever
    // be 1 trailing var arg.
    #[position(last, 0..=1)]
    OptionalTrailingVarArg(Vec<String>),
}
```

## Options struct

The options struct has just one fundamental attribute: `map`. It works much like
a `match` expression (in fact, that's what it expands to). Furthermore, it's
possible to define defaults on fields.

```rust
#[derive(Options, Default)]
struct Settings {
    // When a Arg::Foo is parsed, set this field to `true`.
    // Any expression is possible.
    // Any field starts with `Default::default()`.
    #[map(Arg::Foo => true)]
    foo: bool

    // Arg::BarTrue sets this to true, Arg::BarFalse sets this to false.
    // We can have as many arms as we want. For each field, the first 
    // matching arm is applied and the rest is ignored.
    #[map(
        Arg::BarTrue => true,
        Arg::BarFalse => false,
    )]
    bar: bool,

    // We can set a default value with the field attribute.
    #[map(Arg::Baz => false)]
    #[field(default = true)]
    baz: bool,

    // We can also define a env var to read from if available, else
    // the default value will be used.
    #[map(Arg::SomeVar => true)]
    #[field(env = "SOME_VAR", default = false)]
    some_var: bool,
}
```

As a shorthand, there is also a `set` attribute. These fields behave
identically:

```rust
#[derive(Options, Default)]
struct Settings {
    #[map(Arg::Foo(f) => f)]
    bar: u64,

    #[set(Arg::Foo)]
    baz: u64
}
```

## `FromValue` enums

We often want to map values to some enum, we can define this mapping by deriving
`FromValue`:

```rust
#[derive(Default, FromValue)]
enum Color {
    #[value("always", "yes", "force")]
    Always,

    #[default]
    #[value("auto", "tty", "if-tty")]
    Auto,
    
    #[value("never", "no", "none")]
    Never,
}
```
