use std::{collections::BTreeSet, fs};

#[test]
fn embedded_asset_sizes() {
    #[track_caller]
    fn kib(path: &str) -> usize {
        let meta = fs::metadata(path).unwrap();
        (meta.len() as f64 / 1_024.0).round() as usize
    }

    assert_eq!(10, kib("generated/acknowledgements_full.bin"));

    assert_eq!(859, kib("generated/syntaxes-onig-newlines.bin"));
    assert_eq!(804, kib("generated/syntaxes-fancy-newlines.bin"));

    assert_eq!(858, kib("generated/syntaxes-onig-no-newlines.bin"));
    assert_eq!(803, kib("generated/syntaxes-fancy-no-newlines.bin"));

    assert_eq!(45, kib("generated/themes.bin"));
}

#[rustfmt::skip]
const EXPECTED: &[&str] = &[
    // A
    "ActionScript", "Ada", "Apache Conf", "AppleScript", "AsciiDoc (Asciidoctor)",
    "Assembly (x86_64)", "ASP", "AWK",
    // B
    "Bourne Again Shell (bash)", "Batch File", "BibTeX",
    // C
    "C", "C#", "C++", "Cabal", "Clojure", "CMake", "CoffeeScript", "Comma Separated Values",
    "Crontab", "Crystal", "CSS",
    // D
    "D", "Dart", "Diff", "Dockerfile", "DotENV",
    // E
    "Elixir", "Elm", "Email", "Erlang",
    // F
    "F#", "Fish", "Fortran (Fixed Form)",
    // G
    "Git Attributes", "Git Commit", "Git Config", "Git Ignore", "Git Link", "Git Log",
    "Git Mailmap", "Git Rebase Todo",
    "GLSL", "Go", "GraphQL", "Graphviz (DOT)", "Groff/troff", "Groovy",
    // H
    "Haskell", "HTML",
    // I
    "INI",
    // J
    "Java", "Java Server Page (JSP)", "JavaScript", "Jinja2", "JQ", "JSON", "Julia",
    // K
    "Kotlin",
    // L
    "LaTeX", "LaTeX Log", "Lean", "Less", "Lisp", "Literate Haskell", "LLVM", "Lua",
    // M
    "Makefile", "Manpage", "Markdown", "MATLAB", "MediaWiki", "MultiMarkdown",
    // N
    "NAnt Build File", "nginx", "Nim", "Ninja", "Nix",
    // O
    "Objective-C", "Objective-C++", "OCaml", "OCamllex", "OCamlyacc", "orgmode",
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
    // V
    "varlink", "Verilog", "VimL", "Vyper",
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
    // Misc
    "Java Properties", "JavaScript (Rails)", "jsonnet", "Vue Component", "camlp4", "Plain Text",
    "R Console", "SQL (Rails)", "Protocol Buffer (TEXT)", "gnuplot", "HTTP Request and Response",
    "log", "syslog", "Highlight non-printables",
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
