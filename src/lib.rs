//! Dedicated to chasing the [`bat` man](https://github.com/sharkdp)
//!
//! Extra syntax and theme definitions for
//! [`syntect`] including many common ones
//! that are missing from the default set like TOML, TypeScript, and Dockerfile.
//! Curated by the [`bat` Project](https://github.com/sharkdp/bat)
//!
//! ## Example
//!
//! The following
//!
//! ```cmd
//! $ cargo add two-face --features syntect-default-onig
//! ```
//!
//! ```
//! use two_face::re_exports::syntect;
//!
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
//! where `htmlified` displays as
//!
//! <pre style="background-color:#2e3440;">
//! <span style="color:#d8dee9;">[section]
//! </span><span style="color:#81a1c1;">key </span><span style="color:#d8dee9;">= </span><span style="color:#b48ead;">123
//! </span></pre>
//!
//!
//! ## Feature Flags
//!
//! The feature flags are divided by `syntect`'s underlying regex implementation
//! with [`Oniguruma`](https://github.com/kkos/oniguruma) aka `onig` being the
//! default and [`fancy-regex`](https://github.com/fancy-regex/fancy-regex) aka
//! `fancy` as an alternative pure-Rust implementation. `fancy`: however, doesn't
//! support all of the features used by some of the syntax definitions, so some of
//! the defintions are excluded when `fancy` is selected\* to keep the regex
//! compilation infallible. This means that it's important to match whichever regex
//! implementation `syntect` is using
//!
//! _\* This is also why fancy's bundled syntax definitions are smaller than onig's_
//!
//! default: `syntect-onig`
//!
//! | Feature | Desc. |
//! | :---: | :--- |
//! | `syntect-onig` / `syntect-fancy` | Enables the minimal feature set that we require from `syntect` |
//! | `syntect-default-onig` / `syntect-default-fancy` | The mimimal feature sets along with `syntect`'s default feature set (useful when using the `syntect` re-export) |
//!
//! ## Embedded Asset Sizes
//!
//! This crate embeds some reasonably large assets in the final binary in order to
//! work. Luckily the linker is smart enough to discard unused assets, so you
//! generally only pay for what you use
//!
//! For reference here are the sizes associated with their different functions
//!
//! | function | `two-face` (KiB) | `syntect` (KiB) |
//! | ---: | ---: | ---: |
//! | [`acknowledgement::listing()`] | 10 | - |
//! | [`syntax::extra_newlines()`] (onig) | 920 | 360 |
//! | ^^ (fancy) | 865 | 360 |
//! | [`syntax::extra_no_newlines()`] (onig) | 919 | 359 |
//! | ^^ (fancy) | 864 | 359 |
//! | [`theme::extra()`] | 45 | 5 |
//!
//! In short the syntax definitions are the real chonky part, and if you're
//! switching from `syntect` to `two-face`, then you can expect a ~0.5MiB increase
//! in binary size from them (in exchange for _a lot_ of syntax definitions)
//!
//! ## Syntaxes
//!
//! The full listing of all syntaxes included in [`syntax`]
//!
//! - \* Excluded when using the `fancy-regex` implementation
//! - † Included in `syntect`'s bundled defaults
//!
//! |  | Syntax Definition |
//! | :---: | :---: |
//! | A | ActionScript†, Ada, Apache Conf, AppleScript†, AsciiDoc, ASP†, ARM Assembly\*, Assembly (x86\_64), AWK |
//! | B | Bash†, Batch File†, BibTeX† |
//! | C | C†, C#†, C++†, Cabal, CFML, Clojure†, CMake, CoffeeScript, Crontab, Crystal, CSS†, CSV† |
//! | D | D†, Dart, Dockerfile, DotENV, Diff† |
//! | E | Elixir, Elm, Email, Erlang† |
//! | F | F#, Fish, Fortran |
//! | G | Git (commit, config, ignore, etc.)†, GLSL, Go†, GraphQL, Graphviz (DOT)†, Groff/troff†, Groovy† |
//! | H | Haskell†, HTML† |
//! | I | INI |
//! | J | Java†, Javadoc†, Java Server Page (JSP)†, JavaScript†, JavaScript (Babel)\*, Jinja2, JQ, JSON†, Julia |
//! | K | Kotlin |
//! | L | LaTeX†, LaTeX Log†, Lean, LESS, Lisp†, Literate Haskell†, LiveScript\*, LLVM, Lua† |
//! | M | Makefile†, Manpage, Markdown†, MATLAB†, Mediawiki, MutliMarkdown† |
//! | N | NAnt Build File†, Nginx, Nim, Ninja, Nix, NSIS |
//! | O | Objective-C†, Objective-C++†, OCaml†, OCamllex†, OCamlyacc†, Org Mode |
//! | P | Pascal†, Perl†, PHP†, PowerShell\*, Protobuf, Puppet, PureScript, Python† |
//! | Q | QML |
//! | R | R†, Racket, Rd†, Rego, Regular Expression†, Requirements.txt, reStructuredText†, Robot Framework, Ruby†, Ruby Haml†, Ruby on Rails†, Ruby Slim, Rust† |
//! | S | Sass\*, Scala†, SCSS, Salt State SLS\*, SML, Solidity, SQL†, Strace, Stylus, Svelte, Swift, SystemVerilog |
//! | T | Tcl†, Terraform, TeX†, Textile†, Todo.txt, TOML, TypeScript, TypescriptReact |
//! | V | Varlink, Verilog, VimL, Vue, Vyper |
//! | W | WGSL |
//! | X | XML† |
//! | Y | YAML† |
//! | Z | Zig |
//!
//! ## Themes
//!
//! _Note: For visual examples of all of the embedded themes look at the docs for
//! [`theme::EmbeddedThemeName`]_
//!
//! The full listing of themes provided by [`theme`]. Many of these themes
//! only make sense situationally, so you'll likely want to only expose a subset
//!
//! - † Included in `syntect`'s bundled defaults
//!
//! |  | Theme |
//! | :---: | :---: |
//! | 1 | 1337 (aka leet) |
//! | A | Ansi |
//! | B | Base16, Base16-256, Base16-Eighties (dark)†, Base16-Mocha (dark)†, Base16-Ocean (light/dark)† |
//! | C | Coldark (cold/dark aka light/dark) |
//! | D | DarkNeon, Dracula |
//! | G | GitHub, gruvbox (light/dark) |
//! | I | InspiredGitHub† |
//! | M | Monokai Extended (plain, bright, light, and origin) |
//! | N | Nord |
//! | O | One Half (light/dark) |
//! | S | Solarized (light/dark)†, Sublime Snazzy |
//! | T | TwoDark |
//! | V | Visual Studio Dark+ |
//! | Z | Zenburn |
//!
//! ## Legal
//!
//! The embedded syntax definitions and assets also have their own licenses which
//! are compiled into
//! [this markdown file](https://github.com/CosmicHorrorDev/two-face/blob/main/generated/acknowledgements_full.md)
//! along with programmatic in the [`acknowledgement`] module

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
pub struct ReadmeDoctests;

pub mod acknowledgement;
// Unstable extra code that we use for fuzzing
#[cfg(fuzzing)]
#[doc(hidden)]
pub mod fuzz;
pub mod syntax;
pub mod theme;

/// Dependency re-exports for user's convenience
///
/// # `syntect`
///
/// By default `two-face` uses the minimal feature set from `syntect` required for things to work,
/// but the default features can be toggled on with the `syntect-default-onig` and
/// `syntect-default-fancy` feature flags (depending on which syntect regex implementation you're
/// using). If you need more granular features than the ones provided then you should probably
/// depend directly on `syntect` instead
pub mod re_exports {
    pub use syntect;
}

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
two-face = ...

or

# `fancy-regex` based
[dependencies]
syntect = { version = ..., default-features = false, features = ["default-fancy"]
two-face = { version = ..., default-features = false, features = ["syntect-fancy"] }"#
);

/// Returns a link to a page listing acknowledgements for all syntax and theme definitions
///
/// Available without having to bundle all of the acknowledgement info in your binary
///
/// ```
/// assert_eq!(
///     two_face::acknowledgement_url(),
///     "https://github.com/CosmicHorrorDev/two-face/blob/v0.4.3/generated/acknowledgements_full.md"
/// );
/// ```
pub fn acknowledgement_url() -> &'static str {
    concat!(
        "https://github.com/CosmicHorrorDev/two-face/blob/v",
        env!("CARGO_PKG_VERSION"),
        "/generated/acknowledgements_full.md",
    )
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
