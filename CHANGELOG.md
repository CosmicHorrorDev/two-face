# Version 0.3.0

Only a couple of very small breaking changes:

- The embedded theme set now includes all of syntect's default themes instead of
  just bat's
- The themes returned by `EmbeddedLazyThemeSet::theme_names()` are now
  alphabetized

# Version 0.2.0

## Feature Flags

There are a couple of changes around feature flags:

- All the feature flags revolving around limited which embedded assets get
  pulled in have been ripped out. Luckily the linker is smart enough to strip
  out these unused assets which makes things a lot simpler
- There are new features to indicate which regex implementation you're using
  with `syntect`. Some syntaxes use features that are only provided by `onig`
  and not `fancy-regex`, so matching `two-face` and `syntect`'s implementation
  keeps regex compilation infallible

## Acknowledgments

Acknowledgments now includes the full listing for all syntaxes and themes
regardless of what is used. There are still individual methods to distinquish
which acknowledgments are for what though

## Syntaxes

The `two_face::syntax::extra()` function is now
`two_face::syntax::extra_no_newlines()` and `two_face::extra::extra_newlines()`
has been added. This mirrors `syntect`'s `load_defaults_nonewlines()` and
`load_defaults_newlines()`

## Themes

`two_face::theme::extra()` now includes an `EmbeddedLazyThemeSet`. This is just
a wrapper around a `LazyThemeSet` tailored toward knowing all of the embedded
themes
