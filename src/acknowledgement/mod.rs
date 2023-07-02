mod core_types;
pub use core_types::{Acknowledgements, License, LicenseType};

impl Acknowledgements {
    #[cfg_attr(docsrs, doc(cfg(feature = "extra-syntax")))]
    #[cfg(feature = "extra-syntax")]
    pub fn for_syntaxes(&self) -> &[License] {
        &self.for_syntaxes
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "extra-theme")))]
    #[cfg(feature = "extra-theme")]
    pub fn for_themes(&self) -> &[License] {
        &self.for_themes
    }
}

#[cfg(all(feature = "extra-syntax", feature = "extra-theme"))]
pub fn listing() -> Acknowledgements {
    // The generated data is a bit smaller when we can bundle it together
    syntect::dumps::from_binary(include_bytes!("../../generated/acknowledgements_full.bin",))
}

#[cfg(not(all(feature = "extra-syntax", feature = "extra-theme")))]
pub fn listing() -> Acknowledgements {
    #[cfg(feature = "extra-syntax")]
    let for_syntaxes = syntect::dumps::from_binary(include_bytes!(
        "../../generated/acknowledgements_syntaxes.bin",
    ));
    #[cfg(not(feature = "extra-syntax"))]
    let for_syntaxes = Vec::new();

    #[cfg(feature = "extra-theme")]
    let for_themes =
        syntect::dumps::from_binary(include_bytes!("../../generated/acknowledgements_theme.bin"));
    #[cfg(not(feature = "extra-theme"))]
    let for_themes = Vec::new();

    Acknowledgements {
        for_themes,
        for_syntaxes,
    }
}
