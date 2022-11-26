TODO list for design:

- Parsing
    - Flags
        - [x] Flags based on `bool`
        - [ ] Flags based on `enum`
        - [ ] Inferring long flags/options
    - Options
        - [ ] Options (with `Option<T>`)
        - [ ] Options with default values
        - [ ] Options with optional values
        - [ ] Options with multiple values/occurrences
        - [ ] Options based on `enum`
        - [ ] Option values parsing?
        - [ ] Default values from environment variables
    - [ ] Error handling
    - [ ] Position arguments
    - [ ] Trailing var arg
    - [ ] Conflicts
    - [ ] `dd` style arguments? (Probably not worth it)
- Help & Version
    - [ ] Help
    - [ ] Usage
    - [ ] Version
    - [ ] Help and version override default flags
    - [ ] Defining metadata (author, license, etc.)
    - [ ] Defining short/long help
    - [ ] Help strings from external markdown file
- [ ] Completion
- [ ] `mdbook` generation
- [ ] `man` generation

---

In uutils, we have been using `clap` as our argument parser. `clap` is an
excellent library and still improving at a steady rate. However, our needs are
extremely specific and we have to work around several limitations of `clap`,
e.g. by manually checking certain indices.

One such example is the `-f` flag of `rm`, which does two things:

1. It suppresses warnings.
2. It disables interactive prompts.

This means that the behaviour or `rm -fi` (`-i` is interactive), is different
from `rm -if`. In the first one, warnings are suppressed and we have
interactive prompts, in the second one, we have the behaviour of `rm -f`.
I'm gonna call this "partial overrides".

Let's imagine a shiny future where we have a derive API for argument parsing
that supports this case and other weird cases in `coreutils`. This is purely
hypothetical for now, as both implementation and migration to this new API,
would take a long time.

I'll be distinguishing three types of parameters:
1. Flags: arguments without a value, just present or not, like `--opt`.
2. Options: arguments with a value like `--opt=value` or `--opt value`.
3. Positional arguments.

# The big idea

`clap` and other `derive`-based API's use an approach to the definition of the
arguments where each field is an argument. This leads to a 1-1 relation between
arguments and settings (n-1 if you count aliases). I think this is great for most
applications, but it leads to friction when implementing the `coreutils`. In
the GNU coreutils, each arguments is lexed and then applied to the settings.
Hence, some utils have a n-m relation between arguments and settings.

The solution is then to decouple arguments and how they map to settings.
Instead, we define a `Settings` struct and define through attributes how the
flags map to these settings.

As a small teaser, consider this example:

```rust
struct Settings {
    #[flag(-x, -z)]
    a: bool,
    #[flag(-x, -y, -z)]
    b: bool,
    #[flag(-y, -z)]
    c: bool,
}
```

The flags `-x`, `-y` and `-z` all set multiple values, hence we have a n-m
relationship. The equivalent code in `clap` would require a manual mapping
from the parsed arguments to the settings.

Arguably, the example above is bad API design and `clap`'s approach is
therefore better. In uutils, however, we don't have the luxury to define our
own API; we have to follow the GNU utils to maintain compatibility.

# Simple Flags

I'm calling our library `magic` for now, because it does not exist. Let's start
by supporting flags.

```rust
use magic::{Options};

#[derive(Options, Default)]
struct Settings {
    // -o and --one are implicit
    // if the field name has only one character, then only the short flag is
    // implied
    #[flag]
    one: bool,
    
    // Only -t, not --two
    #[flag(-t)]
    two: bool,
    
    // Only --three, not -t
    #[flag(--three)]
    three: bool,

    // Explicit form of `#[flag]`
    #[flag(-f, --four)]
    four: bool,
    
    // We can define as many flags as we want
    #[flag(-a, -b, --five, --six)]
    five: bool,
    
    // Cannot be set by an argument
    six: bool,
}
```

This would expand to:

```rust
struct Settings {
    one: bool,
    two: bool,
    three: bool,
    four: bool,
    five: bool,
    six: bool,
}

impl Options for Settings {
    fn parse(args: impl Iterator<&OsStr>) -> Result<Self, magic::Error> {
        let mut self = Self::default();

        use magic::Token::{Short, Long};
        for s in lexer(args) {
            match s {
                Short('o') | Long("one") => self.one = true,
                Short('t') => self.two = true,
                Long("three") => self.three = true,
                Short('f') | Long("four") => self.four = true,
                Short('a') | Short('b') | Long("five") | Long("six") => {
                    self.five = true,
                }
            }
        }
        self
    }
}
```

Now here is the nice part for our `rm` usecase. Nothing is stopping us for
specifying the same flag multiple times. `rm` with just those flags is simply:

```rust
struct Settings {
    #[flag]
    force: bool,

    #[flag]
    #[hidden_flag(-f, --force, false)]
    interactive: bool
}
```

Where `hidden_flag` functions just like `flag`, but will be hidden in the
`--help` (more on that later).

> **Note**: the rules for the arguments of `(hidden_)flag` are `-` specifies
> a short flag, `--` a long flag and another value is a value to set.

Which expands to something along these lines (though probably less pretty
because it will be autogenerated):

```rust
impl Options for Settings {
    fn parse(args: impl Iterator<&OsStr>) -> Result<Self, magic::Error> {
        let mut self = Self::default();
        for s in lex(args) {
            match s {
                Short('f') | Long("force") => {
                    self.force = true;
                    self.interactive = false;
                }
                Short('i') | Long("interactive") => {
                    self.interactive = true;
                }
            }
        }
        self
    }
}
```

# Help generation

We can do help generation like `clap`'s derive API, but we could build in
markdown support from the start. Note that help is not added by default and
must by added explicitly.

```rust
/// help for the entire application goes here
#[derive(Options, Default)]
#[help(-h, --help)] // or #[help]
struct Settings {
    /// Some help in *markdown*
    #[flag]
    force: bool
}
```

The markdown is parsed at compile-time, but rendered at runtime to fit the
terminal width. The markdown parser will therefore not be part of the
executable.

> **Note**: markdown help should probably be feature gated. The alternative is
> just printing the string.

But, because we have the n-m relationship, not each flag has a nicely defined
field in the struct. To document these flags we can create dummy fields, which
will be optimized away at runtime by the compiler. The type can be anything,
but making it zero-size is probablty a good idea (so either `()` or
`PhantomData`). A `dummy_flag` attribute then specifies to treat this as a
`flag` for help generation, but not for parsing.

```rust
#[derive(Options, Default)]
struct Settings {
    /// Some help for `-a`
    #[flag]
    #[hidden_flag(-c)]
    a: bool,
    
    /// Some help for `-b`
    #[flag]
    #[hidden_flag(-c)]
    b: bool,
    
    /// Some help for `-c`
    #[dummy_flag]
    c: (),
}
```

So the `--help` would look roughly like this

```txt
  -a  Some help for `-a`
  -b  Some help for `-b`
  -c  Some help for `-c`
```

Similarly, we can do something like this:

```rust
#[derive(Options, Default)]
struct Settings {
    /// Enable long format
    #[flag(-l, --long)]
    #[hidden_flag(-s, --short, false)]
    long_format: bool,
    
    /// Disable long format
    #[dummy_flag(-s, --short)]
    short_format: ()
}
```

Note that this has overriding behaviour by default. That is, `-sl` will use the
long format and `-ls` will use the short format. I'm leaving conflicts for
later because most coreutils have overriding behaviour.

There's also a chance for
this to be error prone, because there's no syncing of the flags between the
different attributes. Another possible representation could be:

```rust
#[derive(Options)]
struct Settings {
    /// Enable long format
    #[flag(-l, --long)]
    #[flag(-s, --short, false, help = "Disable long format")]
    long_format: bool,
}
```

We'll have to see what works best in practice.

# Options

Often options take a few possible values, which we can model with enums:

```rust
#[derive(FromOpt, Default)]
enum Color {
    #[value]
    Always,

    #[default]
    #[value]
    Auto,

    #[value]
    Never
}

#[derive(Options, Default)]
struct Settings {
    #[option(--color)]
    color: Color; 
}
```

Sometimes we need flags that are shortcuts for option values, we can support
this by adding `flag` attributes to the `enum`.

```rust
#[derive(FromOpt, Default)]
enum Sort {
    #[flag(-U)]
    #[value]
    None,
    
    #[default]
    Name,
    
    #[flag(-S)]
    #[value]
    Size,
    
    #[flag(-t)]
    #[value]
    Time,
    
    #[flag(-v)]
    #[value]
    Version,

    #[flag(-X)]
    #[value]
    Extension,

    #[value]
    Width,
}

#[derive(Options, Default)]
struct Settings {
    #[option(--sort)]
    sort: Sort; 
}
```

This models the `--sort`, `-U`, `-S`, `-t`, `-v` `-X` options of `ls`, and
though it may look complex, I would argue it is very efficient for 5 flags and
an option with 6 possible values and a default value. Note that we were able to
also specify that `Width` does not have a flag and that `Name` cannot be
expressed at all, apart from being the default.

We can also express multiple values per variant, take for instance the `format` 
argument of `ls`:

```rust
#[derive(FromOpt, Default)]
enum Format {
    #[value("long", "verbose")]
    Long,

    #[value]
    SingleColumn,

    #[default]
    #[value("columns", "vertical")]
    Columns,

    #[value("across", "horizontal")]
    Across,

    #[value]
    Commas,
}

#[derive(Options, Default)]
struct Settings {
    #[option(--format)]
    format: Format,
}
```

Let's expand on this example with `-o`, `-n` and `-g`:

```
  -g                      like -l, but do not list owner
  -o                      like -l, but do not list group information
  -n, --numeric-uid-gid   like -l, but list numeric user and group IDs
```

The text here is deceptively simple, because they can stack and they are not 
overridden by `-l`. Here's how we can handle them:

```rust

#[derive(FromOpt, Default)]
enum Format {
    #[flag(-l)]
    #[hidden_flag(-g, -o, -n, --numeric-uid-gid)]
    #[value("long", "verbose")]
    Long,

    #[value]
    SingleColumn,

    #[default]
    #[flag(-C)]
    #[value("columns", "vertical")]
    Columns,
    
    #[flag(-x)]
    #[value("across", "horizontal")]
    Across,

    #[value]
    #[flag(-m)]
    Commas,
}

#[derive(Options, Default)]
struct Settings {
    #[flag(-g)]
    long_hide_owner: bool,

    #[flag(-o)]
    long_hide_group: bool,

    #[flag(-n, --numeric-uid-gid)]
    long_numeric_id: bool,

    #[option(--format)]
    format: Format,
}
```

There's one final complication: `-1` is ineffective after `--long`, which is 
actually distinct from `--format=single-column`. Here's a way to handle that,
even though I'm not very happy with it:

```rust
enum Format {
    #[value]
    #[flag(-l)]
    Long,

    #[default]
    #[value]
    #[flag(-C)]
    Columns,
}

struct Settings {
    #[flag(-1, -l)]
    #[hidden_flag(-C, false)]
    #[option(--format=single-column)]
    single_column: bool,
    
    #[option(--format)]
    format: Format,
}
```

We would now display a single column if `format == Format::SingleColumn` or 
`single_column == true`. Luckily, there's only one such case that I know of 
that needs this.

