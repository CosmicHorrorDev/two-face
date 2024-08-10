mod utils;

use utils::TwoFaceAsset;

#[test]
fn asset_check() {
    let _ = two_face::theme::extra();
    let _ = two_face::acknowledgement::listing();
    let _ = two_face::syntax::extra_newlines();
    let _ = two_face::syntax::extra_no_newlines();

    let common_assets = &[TwoFaceAsset::Themes, TwoFaceAsset::AckFull];
    #[allow(unused)]
    #[cfg(feature = "syntect-fancy")]
    let extra_assets = &[
        TwoFaceAsset::SynFancyNewlines,
        TwoFaceAsset::SynFancyNoNewlines,
    ];
    // If both ^^ and vv features are set then vv takes precedence
    #[allow(unused)]
    #[cfg(feature = "syntect-onig")]
    let extra_assets = &[
        TwoFaceAsset::SynOnigNewlines,
        TwoFaceAsset::SynOnigNoNewlines,
    ];
    utils::linker_strips_all_but(
        common_assets
            .iter()
            .chain(extra_assets.iter())
            .cloned()
            .map(Into::into),
    );
}
