//! Contains extra syntax definitions

use syntect::parsing::SyntaxSet;

/// Returns a [`SyntaxSet`] with plenty of extra syntax definitions compared to the default
///
/// Note: This includes all of `syntect`'s embedded syntax definitions
///
/// # Example
///
/// ```
/// // TOML and TypeScript and Dockerfiles oh my!
/// let syn_set = two_face::syntax::extra();
/// let toml = syn_set.find_syntax_by_name("TOML").unwrap();
/// let type_script = syn_set.find_syntax_by_name("TypeScript").unwrap();
/// let dockerfile = syn_set.find_syntax_by_name("Dockerfile").unwrap();
/// ```
pub fn extra() -> SyntaxSet {
    // TODO: expose newlines and no newlines variants through separate feature flags
    syntect::dumps::from_uncompressed_data(include_bytes!("../generated/syntaxes.bin")).unwrap()
}
