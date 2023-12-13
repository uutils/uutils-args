# `uutils-args` Design Docs

This is a series of design documents, explaining the various design goals and
decisions. Before diving in, let's lay out the design goals of this project.

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

## Pages

1. [Arguments in coreutils](arguments_in_coreutils.md)
2. [Problems with `clap` and other parsers](problems_with_clap.md)
3. [Library design](design.md) (TODO once the design settles)
