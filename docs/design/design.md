# Design

This module contains some documents about the design of this library. In particular, it details the different kinds of arguments that are present in the coreutils and the difficulties that `clap` presents when implementing these arguments.

The primary design considerations of this library are:

- Must support all options in GNU coreutils.
- Must support a many-to-many relationship between options and settings.
- Must have a convenient derive API.
- Must support help strings from file.
- Code must be "greppable" (e.g. search file for `--all` to find the code for
  that argument).
- Maintainability is more important than terseness.
- With a bit of luck, it will be smaller and faster than `clap`, because we have
  fewer features to support.
- Use outside uutils is possible but not prioritized. Hence, configurability
  beyond the coreutils is not necessary.
- Errors must be at least as good as GNU's, but may be different (hopefully
  improved).

## Chapters

1. [Arguments in the coreutils](design::coreutils)
2. [Problems with `clap`](design::problems)
