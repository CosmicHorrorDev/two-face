//! Contains extra syntax definitions

use syntect::{dumps, parsing::SyntaxSet};

// TODO: can we rely on linker garbage collection to prune embedded data instead of features

/// Returns a [`SyntaxSet`] with plenty of extra syntax definitions compared to the default
///
/// Note: This includes all of `syntect`'s embedded syntax definitions
///
/// # Example
///
/// ```
/// // TOML and TypeScript and Dockerfiles oh my!
/// let syn_set = two_face::syntax::extra_no_newlines();
/// let toml = syn_set.find_syntax_by_name("TOML").unwrap();
/// let type_script = syn_set.find_syntax_by_name("TypeScript").unwrap();
/// let dockerfile = syn_set.find_syntax_by_name("Dockerfile").unwrap();
/// ```
pub fn extra_no_newlines() -> SyntaxSet {
    #[cfg(feature = "syntect-onig")]
    let bytes = include_bytes!("../generated/syntaxes-onig-no-newlines.bin");
    #[cfg(not(feature = "syntect-onig"))]
    let bytes = include_bytes!("../generated/syntaxes-fancy-no-newlines.bin");

    dumps::from_uncompressed_data(bytes).unwrap()
}

pub fn extra_newlines() -> SyntaxSet {
    #[cfg(feature = "syntect-onig")]
    let bytes = include_bytes!("../generated/syntaxes-onig-newlines.bin");
    #[cfg(not(feature = "syntect-onig"))]
    let bytes = include_bytes!("../generated/syntaxes-fancy-newlines.bin");

    dumps::from_uncompressed_data(bytes).unwrap()
}
