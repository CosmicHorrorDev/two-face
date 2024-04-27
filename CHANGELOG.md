# Version 0.4.0

## Breaking Changes

- Renamed `EmbeddedThemeName::SubmlimeSnazzy` ->
  `EmbeddedThemeName::SublimeSnazzy`

## Features

- Our `syntect` dependency is now re-exported under `two_face::re_exports::syntect`
- New `syntect-default-onig` / `syntect-default-fancy` features toggle on the
  underlying `default-onig` / `default-fancy` features making the syntect
  re-export decently useful

## Docs

- Both the README and docs landing page got an overhaul
- The `theme` module got some TLC including a demo of all of the embedded themes
- We now have relevant categories in our `Cargo.toml`
- Our feature flags are now documented in a way that lib.rs understands

## Internal

- Code coverage is now tracked on codecov.io
- Dependabot now handles updating our CI actions
- The `unstable` feature flag was removed now that `syntect-default-onig` fills
  the same purpose in an official capacity
- The specific markdown acknowledgement format was changed to be more succinct
- We now support applying patches on top of those provided with `bat` including
  an initial demo stripping comments from Markdown's regexes

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
