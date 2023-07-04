//! Dedicated to chasing the [`bat` man](https://github.com/sharkdp)
//!
//! Extra syntax and theme definitions for
//! [`syntect`](https://docs.rs/syntect/latest/syntect/) including many common ones that are missing
//! from the default set like TOML, TypeScript, and Dockerfile. `syntect` embeds ~0.5 MiB of static
//! assets for the default themes and styles, so it's best to use `default-features = false` when
//! using this crate to avoid pulling in unused embedded assets.
//!
//! # Example
//!
//! ```toml
//! [dependencies]
//! syntect = { version = "0.5.0", default-features = false, features = ["html"] }
//! two-face = "0.1.0"
//! ```
//!
#![cfg_attr(all(feature = "extra-syntax", feature = "extra-theme"), doc = "```")]
#![cfg_attr(
    not(all(feature = "extra-syntax", feature = "extra-theme")),
    doc = "```ignore"
)]
//! const TOML_TEXT: &str = "\
//! [section]
//! key = 123
//! ";
//!
//! fn main() {
//!     let syn_set = two_face::syntax::extra();
//!     let theme_set = two_face::theme::extra();
//!
//!     let syn_ref = syn_set.find_syntax_by_extension("toml").unwrap();
//!     let theme = theme_set.get("Nord").unwrap();
//!     let htmlified = syntect::html::highlighted_html_for_string(
//!         TOML_TEXT,
//!         &syn_set,
//!         syn_ref,
//!         theme
//!     ).unwrap();
//!
//!     // Creates HTML that displays as vv
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
//! This library makes heavy use of feature flags to keep embedded assets and dependencies slim
//!
//! | Flag | Description |
//! | :---: | :--- |
//! | `default` | Enables the `extra-syntax` and `extra-theme` features (akin to `syntect`'s default |
//! | `extra-syntax` | Provides extra syntax definitions including `syntect`s default (no need to worry about juggling both). Currently this contains 196 definitions including some exotic ones |
//! | `extra-theme` | Provides extra theme definitions and the [`LazyThemeSet`][theme::LazyThemeSet] type |
//! | `acknowledgement` | Includes license acknowledgements for all the embedded content that requires acknowledgement. [`acknowledgement_url()`], which is always included in the library, includes all license acknowledgements whether they are required or not (because we don't have to care about bloating binary sizes) |
//!
//! # Sizes
//!
//! Embedding all assets can bulk up the size of your binary a bit of course
//!
//! | Feature Flag | Size (KiB) |
//! | :---: | ---: |
//! | `extra-syntax` | `865.2` |
//! | `extra-theme` | `39.3` |
//! | `acknowledgement` w/ `extra-syntax` | `8.9` |
//! | `acknowledgement` w/ `extra-theme` | `1.7` |
//! | `acknowledgement` w/ both | `9.9` |

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg_attr(docsrs, doc(cfg(feature = "acknowledgement")))]
#[cfg(feature = "acknowledgement")]
pub mod acknowledgement;
#[cfg_attr(docsrs, doc(cfg(feature = "extra-syntax")))]
#[cfg(feature = "extra-syntax")]
pub mod syntax;
#[cfg_attr(docsrs, doc(cfg(feature = "extra-theme")))]
#[cfg(feature = "extra-theme")]
pub mod theme;

/// Returns a link to a page listing acknowledgements for all syntax and theme definitions
///
/// Available regardless of the `acknowledgement` feature, so that you can give credit without
/// needing to embed more assets
pub fn acknowledgement_url() -> &'static str {
    "https://github.com/CosmicHorrorDev/two-face/blob/v0.1.0/generated/acknowledgements_full.md"
}

// TODO: add more extensive tests later
#[cfg(test)]
mod tests {
    // The serialized data is in the right structure
    #[test]
    fn sanity() {
        #[cfg(feature = "acknowledgement")]
        super::acknowledgement::listing();
        #[cfg(feature = "extra-syntax")]
        super::syntax::extra();
        #[cfg(feature = "extra-theme")]
        super::theme::extra();
    }
}
