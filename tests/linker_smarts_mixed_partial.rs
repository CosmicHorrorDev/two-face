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
    use syntect::parsing::SyntaxSet;
    use utils::{SyntectAsset, TwoFaceAsset};

    let _ = two_face::acknowledgement::listing();
    let _ = two_face::theme::extra();
    let _ = SyntaxSet::load_defaults_newlines();

    let expected = [
        TwoFaceAsset::AckFull.into(),
        TwoFaceAsset::Themes.into(),
        SyntectAsset::SynNewlines.into(),
    ];
    utils::linker_strips_all_but(expected.iter().cloned());
}
