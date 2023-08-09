TODO: just keeping track of work that has a messy commit history. Clean up
presention before publishing a release

- Acknowledgments now includes the full whether syntax/theme is set or not
- The default syntax set is now appropriately marked as being for no newlines
    - Feature: `extra-syntax` -> `extra-syntax-no-newlines`
    - Function: `two_face::syntax::extra()` ->
      `two_face::syntax::extra_no_newlines()`
- New feature flag for using a syntax set for lines including newlines
    - Feature: `extra-syntax-newlines`
    - Function: `two_face::syntax::extra_newlines()`
- `two_face::theme::extra()` now returns an `EmbeddedLazyThemeSet` which is a
  newtype wrapper around a `LazyThemeSet` with methods tailored towards just the
  embedded themes. `LazyThemeSet` is still provided for people to use
- `syntect`'s regex impl now must be specified by a feature. Either
  `syntect-fancy` or `syntect-onig` must be set to indicate that `fancy-regex`
  or `onig` is being used as the regex implementation. `syntect` defaults to
  `onig` and will use it if both regex impls are set through features.
  Previously some of the extra syntaxes included would fail when using
  `fancy-regex`. Those problematic syntaxes aren't included when `syntect-fancy`
  is set
- Ripped out the feature flags designed to keep embedded asset usage to a
  minimum. It seems like the linker is smart enough to discard this unused data
