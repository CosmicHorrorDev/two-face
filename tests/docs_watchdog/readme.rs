use std::collections::BTreeSet;

use crate::utils::{Asset, AssetFingerprint, SyntectAsset, TwoFaceAsset};

#[test]
fn embedded_asset_sizes() {
    #[track_caller]
    fn kib(gen: Asset) -> usize {
        let fingerprint: AssetFingerprint = gen.into();
        (fingerprint.size as f64 / 1_024.0).round() as usize
    }

    let tf_ack = kib(TwoFaceAsset::AckFull.into());
    let tf_son = kib(TwoFaceAsset::SynOnigNewlines.into());
    let tf_sfn = kib(TwoFaceAsset::SynFancyNewlines.into());
    let tf_sonn = kib(TwoFaceAsset::SynOnigNoNewlines.into());
    let tf_sfnn = kib(TwoFaceAsset::SynFancyNoNewlines.into());
    let tf_themes = kib(TwoFaceAsset::Themes.into());

    let syn_sn = kib(SyntectAsset::SynNewlines.into());
    let syn_snn = kib(SyntectAsset::SynNoNewlines.into());
    let syn_themes = kib(SyntectAsset::Themes.into());

    let table = format!(
        "\
        | function | `two-face` (KiB) | `syntect` (KiB) |\n\
        | ---: | ---: | ---: |\n\
        | [`acknowledgement::listing()`] | {tf_ack} | - |\n\
        | [`syntax::extra_newlines()`] (onig) | {tf_son} | {syn_sn} |\n\
        | ^^ (fancy) | {tf_sfn} | ^^ |\n\
        | [`syntax::extra_no_newlines()`] (onig) | {tf_sonn} | {syn_snn} |\n\
        | ^^ (fancy) | {tf_sfnn} | ^^ |\n\
        | [`theme::extra()`] | {tf_themes} | {syn_themes} |\n\
        "
    );

    insta::assert_snapshot!(
        table,
        @r"
    | function | `two-face` (KiB) | `syntect` (KiB) |
    | ---: | ---: | ---: |
    | [`acknowledgement::listing()`] | 11 | - |
    | [`syntax::extra_newlines()`] (onig) | 961 | 360 |
    | ^^ (fancy) | 937 | ^^ |
    | [`syntax::extra_no_newlines()`] (onig) | 959 | 359 |
    | ^^ (fancy) | 935 | ^^ |
    | [`theme::extra()`] | 62 | 5 |
    "
    );
}

#[rustfmt::skip]
const EXPECTED: &[&str] = &[
    // A
    "ActionScript", "Ada", "Apache Conf", "AppleScript", "AsciiDoc (Asciidoctor)", "ASP", "AWK",
    // B
    "Bourne Again Shell (bash)", "Batch File", "BibTeX",
    // C
    "C", "C#", "C++", "Cabal", "CFML", "Clojure", "CMake", "CoffeeScript", "Comma Separated Values",
    "Crontab", "Crystal", "CSS",
    // D
    "D", "Dart", "debsources", "Diff", "Dockerfile", "DotENV",
    // E
    "Elixir", "Elm", "Email", "Erlang",
    // F
    "F#", "Fish", "Fortran (Fixed Form)",
    // G
    "GDScript (Godot Engine)", "Git Attributes", "Git Commit", "Git Config", "Git Ignore",
    "Git Link", "Git Log", "Git Mailmap", "Git Rebase Todo", "GLSL", "Go", "GraphQL",
    "Graphviz (DOT)", "Groff/troff", "Groovy",
    // H
    "Haskell", "HTML",
    // I
    "Idris", "INI",
    // J
    "Java", "Java Server Page (JSP)", "JavaScript", "Jinja2", "JQ", "JSON", "Julia",
    // K
    "Kotlin",
    // L
    "LaTeX", "LaTeX Log", "Less", "Lisp", "Literate Haskell", "LiveScript", "LLVM", "Lua",
    // M
    "Makefile", "Manpage", "Markdown", "MATLAB", "MediaWiki", "MultiMarkdown",
    // N
    "NAnt Build File", "nginx", "Nim", "Ninja", "Nix", "NSIS",
    // O
    "Objective-C", "Objective-C++", "OCaml", "OCamllex", "OCamlyacc", "Odin", "orgmode",
    // P
    "Pascal", "Perl", "PHP", "Protocol Buffer", "Puppet", "PureScript", "Python",
    // Q
    "QML",
    // R
    "R", "Racket", "Rd (R Documentation)", "Rego", "Regular Expression", "Requirements.txt",
    "reStructuredText", "Robot Framework", "Ruby", "Ruby Haml", "Ruby Slim", "Ruby on Rails",
    "Rust",
    // S
    "Sass", "Scala", "SCSS", "Solidity", "SML", "SQL", "Strace", "Stylus", "Svelte", "Swift",
    "SystemVerilog",
    // T
    "Tcl", "Terraform", "TeX", "Textile", "Todo.txt", "TOML", "TypeScript", "TypeScriptReact",
    "Typst",
    // V
    "varlink", "Verilog", "VHDL", "VimL", "Vyper",
    // W
    "WGSL",
    // X
    "x86_64 Assembly", "XML",
    // Y
    "YAML",
    // Z
    "Zig",

    // -- Not worth displaying in docs --

    // Various linux files
    "CpuInfo", "fstab", "group", "MemInfo", "passwd", "resolv",
    // Cmake stuff beyond the base
    "CMakeCache", "CMake C Header", "CMake C++ Header",
    // Fortran stuff beyond the base
    "Fortran (Modern)", "Fortran Namelist",
    // Various HTML support for various languages
    "HTML (ASP)", "HTML (EEx)", "HTML (Erlang)", "HTML (Jinja2)", "HTML (Rails)", "HTML (Tcl)",
    "HTML (Twig)",
    // SSH
    "Authorized Keys", "Known Hosts", "Private Key", "SSH Config", "SSHD Config",
    // CSV-like
    "Pipe Separated Values", "Semi-Colon Separated Values", "Separated Values",
    "Tab Separated Values",
    // Go stuff beyond the base
    "Gomod", "Gosum",
    // Lean stuff beyond the base
    "Lean 4",
    // Misc
    "Java Properties", "JavaScript (Rails)", "jsonnet", "Vue Component", "camlp4", "Plain Text",
    "R Console", "SQL (Rails)", "Protocol Buffer (TEXT)", "gnuplot", "HTTP Request and Response",
    "log", "syslog", "Highlight non-printables", "Dockerfile (with bash)", "Command Help",
];

/// Some syntax definitions use regex features that aren't supported by `fancy-regex`
#[rustfmt::skip]
const ONIG_ONLY: &[&str] = &[
    // A
    "ARM Assembly",
    // J
    "JavaScript (Babel)",
    // P
    "PowerShell",
    // S
    "Salt State (SLS)",

    // -- Not worth displaying in docs --

    // Misc
    "VimHelp", "Hosts File",
];

#[test]
fn syntaxes() {
    let extra = if cfg!(feature = "syntect-onig") {
        ONIG_ONLY.iter()
    } else {
        [].iter()
    };
    let mut expected: BTreeSet<_> = EXPECTED.iter().chain(extra).copied().collect();

    // Ensure that `expected` perfectly matches non-hidden syntaxes
    for syntax in two_face::syntax::extra_newlines().syntaxes() {
        if syntax.hidden {
            // Not worth listing in documentation
            continue;
        }

        let name = syntax.name.as_str();
        assert!(expected.remove(name), "Missing syntax: {name}");
    }
    assert!(
        expected.is_empty(),
        "Missing expected syntaxes: {expected:#?}"
    );
}
