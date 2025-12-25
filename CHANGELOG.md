# Version 0.5.0 | 2025-12-25

This update matches `bat`'s recent 0.26.1 release :tada:

## Breaking Changes

- Remove deprecated APIs #148
  - The `Visual Studio Dark+` theme #155
  - `two_face::acknowledgement_url()` which was moved to `two_face::acknowledgement::url()`

## Features

- Update to match `bat@0.26.1` #91 #149
  - Added `debsources`, `GDScript`, `Idris`, `Odin`, `Typst`, `VHDL` syntaxes
  - Added support for Lean 4 to the `Lean` syntax
  - Added `Catppuccin`'s frappe, latte, macchiato and mocha themes
  - Removed the `Visual Studio Dark+` theme

## Deps

- Update outdated dependencies #150 #153

## Docs

- Flesh out some acknowledgments docs #151
- Miscellaneous cleanup #152
- Update motto #154
- Tidy up landing page and license docs #156

# Version 0.4.5 | 2025-12-08

Just a couple of deprecations in preparation for the upcoming `0.5.0` release

## Refactor

- Move `two_face::acknowledgement_url()` to `two_face::acknowledgement::url()` #143

## Docs

- Deprecate `EmbeddedThemeName::VisualStudioDarkPlus` for removal in the next release #144

# Version 0.4.4

This update includes the underlying update to `fancy-regex@0.16.2` which adds
support for `LiveScript` and `Sass` when using the `fancy-regex` backend.

## Deps

- Update outdated dependencies #131

## Internal

- Depend on `serde` and `serde_derive` separately #107

# Version 0.4.3

Fixes builds for dependents that use set `#[cfg(fuzzing)]`

## Fix

- Fix external `#[cfg(fuzzing)]` builds (#61)

# Version 0.4.2

Just a small docs / internal changes update

## Deps

- Update outdated dependencies #54

## Docs

- Sync docs with README tables #51
- Bump our MSRV down to 1.65.0 #55

## Internal

- Bump codecov CI action #52
- Slim down files `include`d in crates.io uploads #53
- Remove dead internal code #56

# Version 0.4.1

## Features

- Update to match `bat`'s 0.25.0 release #49
  - Includes the addition of CFML (ColdFusion Markup Language), NSIS (Nullsoft
    Scriptable Install System), and WGSL (WebGPU Shading Language) syntaxes

## Docs

- Switch README codeblock lang from cmd to console #33
- Use a test to back up the statement on unused assets being stripped #30 #31
- Cleanup README tables #29
- Clarify the MIT license for GitHub #22

## Internal

- Switch the `cargo-xtask` to internally detect the active regex backend instead
  of explicitly being passed #38 #48
- Several CI action version bumps #17 #23 #24 #25 #26 #40 #41 #42 #45
- Update dependencies #35 #44
- Allow automatically generating syntect test metadata #37 #43
- Test embedded acknowledgements compared to full listing #32
- Add a fuzzer to help ensure size optimization equality #19 #21
- Placate `clippy` #20
- Commit our `Cargo.lock` file to help ensure `cargo xtask gen -y` is stable #18

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
