# Argument Types and Behaviour in Coreutils

The coreutils are specified by POSIX and have various implementations. We want
to be compatible with the GNU implementation. Generally, these utils use
`getopt_long` function provided by GNUlib. This is a fairly simple parser, that
you can repeatedly call to iterate over the options passed to the util.

## Default Behaviour

This construction gives the follow default behaviours:

- `--help` and `--version` are used as the flags for, well, help and version,
  respectively.
- `-h` and `-V` are NOT excepted and sometimes even used for other purposes than
  showing help and version.
- Values with leading hyphens are accepted by default.
- `getopt_long` does not do any checking of conflicting arguments. Hence, all
  arguments have overriding behaviour, including overriding themselves.
- Long options are inferred from unambiguous prefixes. For example, `ls --group`
  is inferred to `ls --group-directories-first` because there is no other long
  option starting with `group`.
- The help string is written by hand and not provided by `getopt_long`. To their
  credit, the GNU authors have put great effort into standardizing these
  strings.

## Many-to-many relationship

In the coreutils, there is a very loose coupling between the arguments and their
effect in the program. Take the snippet from `cat` below, for example. Settings
can be changed by multiple options (e.g. `show_nonprinting` is set by `-t`, `-v`
and `-A`). This leads to a many-to-many relationship: each option can change
multiple settings and each settings can be changed by multiple options.

```C
switch (c) {
    case 't':
      show_tabs = true;
      show_nonprinting = true;
      break;

    case 'v':
      show_nonprinting = true;
      break;

    case 'A':
      show_nonprinting = true;
      show_ends = true;
      show_tabs = true;
      break;

    case 'E':
      show_ends = true;
      break;

    case 'T':
      show_tabs = true;
      break;
}
```

## Argument Types

### Flags

There are many simple flags that do not take any values. For example, the flags
from `cat` above are all flags. They can have both long and short versions
(`--show-nonprinting` & `-v`), but they can also have just one of the two.

Some flags are hidden, like `tail`'s `---presume-input-pipe` option. These
hidden arguments also have 3 leading hyphens.

### Options with values

Some options take values. Most of the time, this is just a long options. Some
examples:

- `ls` has `-w, --width=COLS`, where the value is required for both the short
  and long option.
- `ls` has `-F, --classify[=WHEN]`, where the value is optional for the long
  option and the short option does not take a value.
- `ls` has `--hyperlink[=WHEN]`, which does not have a short version (and an
  optional value).
- `mktemp` has `-p DIR, --tmpdir[=DIR]`, where the value is required for the
  short option and optional for the long option.
- `date` has `-I[FMT], --iso-8601[=FMT]`, where the value is optional for both
  the short and long option.

If the option takes one of several possible values, these values are inferred
from unambiguous prefixes. For example, `ls --color=y` can be used as shorthand
for `ls --color=yes`.

### Positional arguments

Some utils take positional arguments, which might be required.

- `arch` takes no positional arguments.
- `comm FILE1 FILE2` takes 2 required positional arguments.
- `tr SET1 [SET2]` has 1 required and 1 optional positional argument.
- `uniq [INPUT [OUTPUT]]` takes 2 optional positional arguments.
- `ls [FILE]...` takes 0 or more positional arguments.
- `cp SOURCE... DEST` take 1 or more source arguments and 1 required destination
  argument, however, `cp -t DIRECTORY SOURCE...` does not have the destination
  argument.
- `timeout DURATION COMMAND...` takes one 1 required duration and a trailing
  argument of minimal 1 value. Any options appearing after the first value of
  `COMMAND` should be parsed as part of `COMMAND`.
- `who [ FILE | ARG1 ARG2 ]` either takes 1 `FILE` argument or 2 `ARG`
  arguments.

### Deprecated syntax `+N` and `-N`

Some utils (e.g. `head`, `tail`, `kill`, `fold` and `uniq`) support an old
deprecated syntax where numbers can be directly passed as arguments as a
shorthand. For example, `uniq +5` is a shorthand for `uniq -s 5` and `uniq -5`
is short for `uniq -f 5`.

These all behave slightly differently.

1. `head` and `tail` only accept this if it is the first argument and either 1
   or 2 arguments are given.
2. In `fold` the `-N` must be standalone (e.g. `-10b` is rejected), but can
   appear at any position.
3. In `kill`, the same rules as `fold` apply, but it can also be a name instead
   of a number.
4. In `uniq`, the syntax does not need to stand alone and is additive in a weird
   way, because they hack `-22` as `-2 -2` so each flag `-1...-9` multiplies the
   previous by 10 and adds itself. I'm not sure that we need to support this.
   Doing something like what `fold` and `kill` do is probably fine. Also note
   that to make it extra confusing, the `+` variant works like `fold`.
5. `pr` the behaviour is similar to `uniq`.
6. `split` seems to be somewhere between `uniq` and `fold`. It accepts things
   like `-x10x` correctly, but it doesn't do the additive thing from `uniq`
   across multiple occurrences. Basically, it's very clever and cursed.
