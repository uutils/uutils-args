# Problems with `clap` and other parsers

To ensure that this library is an improvement over the current situation, we
need to investigate what we want to change and what to keep from `clap`. In the
process, I'll also discuss some other parsers to see if we can take some
inspiration from them.

Before I continue, I want to note that these are not (always) general problems
with `clap`. They are problems that show up when you want to implement the
coreutils with it. The coreutils have some weird behaviour that you won't have
to deal with in a new project. `clap` is still a great library, and you
should probably use it over this library, unless you need compatibility with GNU
utilities.

## Problem 1: No many-to-many relationship between arguments and settings

This is the biggest issue we have with `clap`. In `clap`, it is assumed that
options do not interfere with each other. This means that _partially overriding_
options are really hard to support. `rm` has `--interactive` and `-f`, which
mostly just override each other, because they set the interactive mode and
decide whether to print warnings. However, `--interactive=never` does not change
whether warnings are printed. Hence, they cannot override completely, because
then these two are **not** identical:

```bash
rm -f --interactive=never
rm    --interactive=never
```

The only way we've come up with to support this in `clap` is by manually
comparing the indices between these options, which is not very nice.

This can get very complicated, as is the case in `ls`, where the
[parsing of the format is very strange and error-prone](https://github.com/uutils/coreutils/blob/03710a180eb273e566b52c59ac54715844380e0c/src/uu/ls/src/ls.rs#L432-L505).

## Problem 2: We can't use the derive API

This is mostly due to the previous problem, but because arguments usually change
multiple settings, we cannot use the derive API of `clap` in most cases. We
could go for a hybrid between the derive and builder APIs, which `clap` does
support, but that feels overly complicated. Hence, we stuck with the builder
API.

## Problem 3: Wrong defaults

The defaults of clap are often not what we want them to be. One might argue
`clap`'s defaults are better, but we're aiming for compatibility with coreutils,
so we have no choice but to override them. Here are a few examples:

|                       | `clap` defaults              | coreutils       |
| --------------------- | ---------------------------- | --------------- |
| help flags            | `-h` and `--help`            | `--help`        |
| version flags         | `-V` and `--version`         | `--version`     |
| Long option inference | Optional                     | Always          |
| Conflicting options   | Must be set to override      | Always override |
| Leading hyphens       | Must be set per argument[^1] | Always accepted |

Changing these defaults is sometimes just a single line, but other times it
becomes quite verbose. In particular, setting the options to override becomes
quite verbose in some cases.

[^1]:
    There is a setting to set it for all arguments, but it behaves differently
    than setting it individually and leads to some troubles, due to the differences
    mentioned in the next section.

## Problem 4: Subtle differences

`clap` parses differently than `getopt`. This can be seen with optional values:
`clap` does not require a `=` between the flag and the value, but `getopt` does.
Instead, `clap` checks whether the next argument starts with a hyphen to check
whether the value is the value of the previous option or a new option.
Therefore, unless we tell `clap` that a `=` is required it will parse `foo.txt`
as the value to `--color` instead of as a file here:

```bash
ls --color foo.txt
```

Now assume there is some argument `-f`, `--foo` with an optional value. If we do
require `=`, then the behaviour is still not correct, because now `clap` also
requires a `=` for the short option. In the coreutils, however, `=` is never
used for a short option. Hence, the only way to get the desired behaviour is to
create multiple arguments.

But even then, there is no way to tell `clap` to consider the `=` as part of the
value. E.g. `cut -d=` will be parsed as `cut -d''`, which we have to work
around.

It happens quite often that we miss these subtle differences and therefore end
up not being compatible with GNU coreutils. If we do want to do this correctly,
it usually takes changing multiple settings to get the desired result.

## Problem 5: Deprecated syntax of `head`, `tail` and `uniq`

As discussed in the argument types document, these utils support a shorthand
syntax for some options (e.g. `-5` is short for `-s 5`). We have not managed to
implement these nicely with `clap`. Our best efforts try to filter these values
out of the arguments before passing them to `clap`, but it is extremely
difficult to handle all edge-cases.

## Problem 6: Exit codes

In coreutils, different utils have different exit codes when they fail to parse.
For example, `timeout` returns `125`, because the command it calls probably uses
`1` or `2`. There is no way to customize this in `clap`, so we work around it in
uutils and when we opened as issue for it, it was discarded. This makes sense
from `clap`'s perspective, but it shows that the priorities between `clap` and
uutils diverge.

## Problem 7: It's stringly typed

`clap`'s arguments are identified by strings. This leads to code like this:

```rust,ignore
const OPT_NAME: &'static str = "name";

// -- snip --

fn main() {
    let cmd = Command::new(...)
        .arg(
            Arg::new(OPT_NAME)
        );

    // -- snip --
    let name = matches.get_one<String>(OPT_NAME);
}
```

There is no checking at compile time whether `OPT_NAME` has been registered as
an argument and if we wouldn't use a constant, it would be prone to typos. It
also leads to a big list of strings at the top of the file, which is not a big
deal, but a bit annoying.

Of course, we wouldn't have this problem if we were able to use the derive API.

## Problem 8: Reading help string from a file

In `uutils` our help strings can get quite long. Therefore, we like to extract
those to an external file. With `clap` this means that we need to do some custom
preprocessing on this file to extract the information for the several pieces of
the help string that `clap` supports.

## Problem 9: No markdown support

Granted, this is not really a problem, but more of a nice-to-have. We have
online documentation for the utils, based on the help strings and these are
rendered from markdown. Ideally, our argument parser supports markdown too, so
that we can have nicely rendered help strings which have (roughly) the same
appearance in the terminal and online.

## Problem 10: No position-dependent argument-error prioritization

This is the question of which error to print if both `-A` and `-B` are given,
and both are individually an error somehow. In case of the GNU tools, only the
first error is printed, and then the program is aborted.

This also is not really a problem, but since it can be reasonably easily
achieved by simply raising an error during argument application, this enables
matching more closely the exact behavior of the GNU tools.

## Good things about `clap`

Alright, enough problems. Let's praise `clap` a bit, because it's an excellent
library.

- The help text looks great (although I think we should turn off the textwrap
  feature).
- The error messages are very informative and provide a lot of context.
- It has no trouble dealing with invalid UTF-8.
- It is very configurable. The fact that we were able to work around most of our
  issues at all, even though it might have been quite verbose is a great
  accomplishment of `clap`'s developers.
- It has support for generating completion information for many shells.
- We can access its internals to generate our online docs.

## Other parsers

I'll do a quick rundown of other parsers and why they are not well-suited to the
uutils project. I've included anything I could find, including obscure
libraries.

- [`lexopt`](https://github.com/blyxxyz/lexopt)
  - Great but very low-level.
  - No help generation or other fancy features.
  - `uutils-args` actually uses `lexopt` under the hood.
- [`clap_lex`](https://github.com/clap-rs/clap/tree/master/clap_lex)
  - As discussed above, `clap`'s lexing is slightly different from coreutils.
  - Otherwise, it would be interesting to build on top of.
- [`argh`](https://github.com/google/argh)
  - Does not handle invalid UTF-8.
  - It is also not configurable enough.
  - Does not support a many-to-many relationship.
- [`bpaf`](https://github.com/pacak/bpaf)
  - Extremely flexible, even supports `dd`-style.
  - A different configuration between short and long options requires a
    workaround.
  - A many-to-many relation ship is possible, though not very ergonomic.
  - For more information, see: <https://github.com/uutils/uutils-args/issues/17>
- [`gumdrop`](https://github.com/murarth/gumdrop)
  - Does not handle invalid UTF-8.
  - Not configurable enough.
  - Does not have the many-to-many relationship (options map directly to fields
    in a struct).
- [`pico_args`](https://github.com/razrfalcon/pico-args)
  - Interesting, but does not seem to provide much over `lexopt`.
- [`xflags`](https://github.com/matklad/xflags)
  - No different configuration between short and long options.
  - Does not have the many-to-many relationship (options map directly to
    fields).
- [`getopts`](https://github.com/rust-lang/getopts)
  - Was once used by uutils.
  - No help generation.
  - No many-to-many relationship.
- [`getopt`](https://docs.rs/getopt/latest/getopt/) and
  [`libc::getopt`](https://docs.rs/libc/latest/libc/fn.getopt.html)
  - No long options.
- [`getopt_long`](https://docs.rs/getopt-long/0.3.0/getopt_long/)
  - Cumbersome to use.
  - Seems unmaintained.
  - No license.
- [`getopt_long`](https://www.gnu.org/software/gnulib/manual/html_node/getopt_005flong.html)
  from GNUlib
  - We can't use GNU code, because of the GPL.
  - We also do not want to do this, because we don't want to depend on GNUlib.
