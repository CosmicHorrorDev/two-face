mod utils;

#[test]
#[cfg(not(any(feature = "syntect-default-onig", feature = "syntect-default-fancy")))]
#[ignore = "Needs syntect default feature"]
fn asset_check() {
    panic!("This should be ignored >.>");
}
#[test]
#[cfg(any(feature = "syntect-default-onig", feature = "syntect-default-fancy"))]
fn asset_check() {
    use strum::IntoEnumIterator;
    use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};
    use utils::SyntectAsset;

    let _ = SyntaxSet::load_defaults_newlines();
    let _ = SyntaxSet::load_defaults_nonewlines();
    let _ = ThemeSet::load_defaults();

    utils::linker_strips_all_but(SyntectAsset::iter().map(Into::into));
}
