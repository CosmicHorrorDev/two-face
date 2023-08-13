//! Dedicated to chasing the [`bat` man](https://github.com/sharkdp)
//!
//! Extra syntax and theme definitions for
//! [`syntect`](https://docs.rs/syntect/latest/syntect/) including many common ones that are missing
//! from the default set like TOML, TypeScript, and Dockerfile
//!
//! # Example
//!
//! ```toml
//! [dependencies]
//! syntect = "0.5.1"
//! two-face = "0.2.0"
//! ```
//!
//! ```
//! const TOML_TEXT: &str = "\
//! [section]
//! key = 123
//! ";
//!
//! fn main() {
//!     let syn_set = two_face::syntax::extra_newlines();
//!     let theme_set = two_face::theme::extra();
//!
//!     let syn_ref = syn_set.find_syntax_by_extension("toml").unwrap();
//!     let theme = theme_set.get(two_face::theme::EmbeddedThemeName::Nord);
//!     let htmlified = syntect::html::highlighted_html_for_string(
//!         TOML_TEXT,
//!         &syn_set,
//!         syn_ref,
//!         theme
//!     ).unwrap();
//!
//!     // Where `htmlified` displays as vv
//!     # assert_eq!(htmlified, "<pre style=\"background-color:#2e3440;\">\n<span style=\"color:#d8dee9;\">[section]\n</span><span style=\"color:#81a1c1;\">key </span><span style=\"color:#d8dee9;\">= </span><span style=\"color:#b48ead;\">123\n</span></pre>\n");
//! }
//! ```
//!
//! <pre style="background-color:#2e3440;">
//! <span style="color:#d8dee9;">[section]
//! </span><span style="color:#81a1c1;">key </span><span style="color:#d8dee9;">= </span><span style="color:#b48ead;">123
//! </span></pre>
//!
//! # Feature Flags
//!
//! Some embedded syntaxes use features that aren't available with `fancy-regex`. To keep regex
//! compilation infallible it's important to match this library's regex implementation with the one
//! you're using from syntect
//!
//! To use [Oniguruma](https://github.com/kkos/oniguruma) aka `onig`
//!
//! ```toml
//! [dependencies]
//! # `onig` is the default
//! syntect = "0.5.1"
//! two-face = "0.2.0"
//! ```
//!
//! To use [`fancy-regex`](https://github.com/fancy-regex/fancy-regex)
//!
//! ```toml
//! [dependencies]
//! syntect = { version = "0.5.1", default-features = false, features = ["default-fancy"] }
//! two-face = { version = "0.2.0", default-features = false, features = ["syntect-fancy"] }
//! ```

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
pub struct ReadmeDoctests;

pub mod acknowledgement;
pub mod syntax;
pub mod theme;

// Compile error if we're using syntaxes without setting fancy vs onig
#[cfg(not(any(feature = "syntect-onig", feature = "syntect-fancy")))]
compile_error!(
    r#"You must set either the `syntect-onig` or `syntect-fancy` feature matching the regex
implemetation that you're using for `syntect`. `syntect` and `two-face` both default to onig along
with using it if both are present, so you have to use `default-features = false` if you want to use
`fancy-regex`. E.g.

# `onig` based
[dependencies]
syntect = ...
two-face = { version = ..., features = ["extra-syntax-..."] }

or

# `fancy-regex` based
[dependencies]
syntect = { version = ..., default-features = false, features = ["default-fancy"]
two-face = {
    version = ...,
    default-features = false,
    features = ["syntect-fancy", "extra-syntax-..."]
}"#
);

/// Returns a link to a page listing acknowledgements for all syntax and theme definitions
///
/// Available regardless of the `acknowledgement` feature, so that you can give credit without
/// needing to embed more assets
pub fn acknowledgement_url() -> &'static str {
    "https://github.com/CosmicHorrorDev/two-face/blob/v0.2.0/generated/acknowledgements_full.md"
}

// TODO: add more extensive tests later
#[cfg(test)]
mod tests {
    // The serialized data is in the right structure
    #[test]
    fn sanity() {
        super::acknowledgement::listing();
        super::syntax::extra_newlines();
        super::syntax::extra_no_newlines();
        super::theme::extra();
    }
}
