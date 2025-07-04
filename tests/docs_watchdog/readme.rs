use std::{collections::BTreeSet, fs};

use crate::utils::TwoFaceAsset;

#[test]
fn embedded_asset_sizes() {
    #[track_caller]
    fn kib(gen: TwoFaceAsset) -> usize {
        let meta = fs::metadata(gen.rel_path()).unwrap();
        (meta.len() as f64 / 1_024.0).round() as usize
    }

    assert_eq!(10, kib(TwoFaceAsset::AckFull));

    assert_eq!(939, kib(TwoFaceAsset::SynOnigNewlines));
    assert_eq!(884, kib(TwoFaceAsset::SynFancyNewlines));

    assert_eq!(938, kib(TwoFaceAsset::SynOnigNoNewlines));
    assert_eq!(883, kib(TwoFaceAsset::SynFancyNoNewlines));

    assert_eq!(62, kib(TwoFaceAsset::Themes));
}

#[rustfmt::skip]
const EXPECTED: &[&str] = &[
    // A
    "ActionScript", "Ada", "Apache Conf", "AppleScript", "AsciiDoc (Asciidoctor)",
    "Assembly (x86_64)", "ASP", "AWK",
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
    "LaTeX", "LaTeX Log", "Lean", "Less", "Lisp", "Literate Haskell", "LLVM", "Lua",
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
    "Scala", "SCSS", "Solidity", "SML", "SQL", "Strace", "Stylus", "Svelte", "Swift",
    "SystemVerilog",
    // T
    "Tcl", "Terraform", "TeX", "Textile", "Todo.txt", "TOML", "TypeScript", "TypeScriptReact",
    "Typst",
    // V
    "varlink", "Verilog", "VimL", "Vyper",
    // W
    "WGSL",
    // X
    "XML",
    // Y
    "YAML",
    // Z
    "Zig",

    // -- Not worth displaying in docs --

    // Various linux files
    "CpuInfo", "fstab", "group", "hosts", "MemInfo", "passwd", "resolv",
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
    // Misc
    "Java Properties", "JavaScript (Rails)", "jsonnet", "Vue Component", "camlp4", "Plain Text",
    "R Console", "SQL (Rails)", "Protocol Buffer (TEXT)", "gnuplot", "HTTP Request and Response",
    "log", "syslog", "Highlight non-printables", "Dockerfile (with bash)",
];

/// Some syntax definitions use regex features that aren't supported by `fancy-regex`
#[rustfmt::skip]
const ONIG_ONLY: &[&str] = &[
    // A
    "ARM Assembly",
    // J
    "JavaScript (Babel)",
    // L
    "LiveScript",
    // P
    "PowerShell",
    // S
    "Salt State (SLS)", "Sass",

    // -- Not worth displaying in docs --

    // Misc
    "Command Help", "VimHelp",
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
