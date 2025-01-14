# _two-face_

[![build status](https://img.shields.io/github/actions/workflow/status/CosmicHorrorDev/two-face/CI.yml?branch=main)](https://github.com/CosmicHorrorDev/two-face/actions)
[![Crates.io](https://img.shields.io/crates/v/two-face.svg)](https://crates.io/crates/two-face)
[![codecov](https://codecov.io/gh/CosmicHorrorDev/two-face/graph/badge.svg?token=MUORSBCHF2)](https://codecov.io/gh/CosmicHorrorDev/two-face)


Dedicated to chasing the [`bat` man](https://github.com/sharkdp)

Extra syntax and theme definitions for
[`syntect`](https://docs.rs/syntect/latest/syntect/) including many common ones
that are missing from the default set like TOML, TypeScript, and Dockerfile.
Curated by the [`bat` Project](https://github.com/sharkdp/bat)

## Example

The following

```console
$ cargo add two-face --features syntect-default-onig
```

```rust
use two_face::re_exports::syntect;

const TOML_TEXT: &str = "\
[section]
key = 123
";

fn main() {
    let syn_set = two_face::syntax::extra_newlines();
    let theme_set = two_face::theme::extra();

    let syn_ref = syn_set.find_syntax_by_extension("toml").unwrap();
    let theme = theme_set.get(two_face::theme::EmbeddedThemeName::Nord);
    let htmlified = syntect::html::highlighted_html_for_string(
        TOML_TEXT,
        &syn_set,
        syn_ref,
        theme
    ).unwrap();

    println!("{htmlified}");
}
```

will print this

```html
<pre style="background-color:#2e3440;">
<span style="color:#d8dee9;">[section]
</span><span style="color:#81a1c1;">key </span><span style="color:#d8dee9;">= </span><span style="color:#b48ead;">123
</span></pre>
```

## Feature Flags

The feature flags are divided by `syntect`'s underlying regex implementation
with [`Oniguruma`](https://github.com/kkos/oniguruma) aka `onig` being the
default and [`fancy-regex`](https://github.com/fancy-regex/fancy-regex) aka
`fancy` as an alternative pure-Rust implementation. `fancy`: however, doesn't
support all of the features used by some of the syntax definitions, so some of
the defintions are excluded when `fancy` is selected\* to keep the regex
compilation infallible. This means that it's important to match whichever regex
implementation `syntect` is using

_\* This is also why fancy's bundled syntax definitions are smaller than onig's_

default: `syntect-onig`

| Feature | Desc. |
| :---: | :--- |
| `syntect-onig` / `syntect-fancy` | Enables the minimal feature set that we require from `syntect` |
| `syntect-default-onig` / `syntect-default-fancy` | The mimimal feature sets along with `syntect`'s default feature set (useful when using the `syntect` re-export) |

## Embedded Asset Sizes

This crate embeds some reasonably large assets in the final binary in order to
work. Luckily [the linker is smart enough to discard unused assets](https://github.com/CosmicHorrorDev/two-face/blob/0979a0dd5faf2197f6c37ee2194e20bbf0b77ce7/tests/linker_smarts_mixed_partial.rs#L11-L25), so you
generally only pay for what you use

For reference here are the sizes associated with their different functions

| function | `two-face` (KiB) | `syntect` (KiB) |
| ---: | ---: | ---: |
| [`acknowledgement::listing()`](https://docs.rs/two-face/latest/two_face/acknowledgement/fn.listing.html) | 10 | - |
| [`syntax::extra_newlines()`](https://docs.rs/two-face/latest/two_face/syntax/fn.extra_newlines.html) (onig) | 920 | 360 |
| ^^ (fancy) | 865 | 360 |
| [`syntax::extra_no_newlines()`](https://docs.rs/two-face/latest/two_face/syntax/fn.extra_no_newlines.html) (onig) | 919 | 359 |
| ^^ (fancy) | 864 | 359 |
| [`theme::extra()`](https://docs.rs/two-face/latest/two_face/theme/index.html) | 45 | 5 |

In short the syntax definitions are the real chonky part, and if you're
switching from `syntect` to `two-face`, then you can expect a ~0.5MiB increase
in binary size from them (in exchange for _a lot_ of syntax definitions)

## Syntaxes

The full listing of all syntaxes included in [`two_face::syntax`](https://docs.rs/two-face/latest/two_face/syntax/index.html)

- \* Excluded when using the `fancy-regex` implementation
- † Included in `syntect`'s bundled defaults

|  | Syntax Definition |
| :---: | :---: |
| A | ActionScript†, Ada, Apache Conf, AppleScript†, AsciiDoc, ASP†, ARM Assembly\*, Assembly (x86\_64), AWK |
| B | Bash†, Batch File†, BibTeX† |
| C | C†, C#†, C++†, Cabal, CFML, Clojure†, CMake, CoffeeScript, Crontab, Crystal, CSS†, CSV† |
| D | D†, Dart, Dockerfile, DotENV, Diff† |
| E | Elixir, Elm, Email, Erlang† |
| F | F#, Fish, Fortran |
| G | Git (commit, config, ignore, etc.)†, GLSL, Go†, GraphQL, Graphviz (DOT)†, Groff/troff†, Groovy† |
| H | Haskell†, HTML† |
| I | INI |
| J | Java†, Javadoc†, Java Server Page (JSP)†, JavaScript†, JavaScript (Babel)\*, Jinja2, JQ, JSON†, Julia |
| K | Kotlin |
| L | LaTeX†, LaTeX Log†, Lean, LESS, Lisp†, Literate Haskell†, LiveScript\*, LLVM, Lua† |
| M | Makefile†, Manpage, Markdown†, MATLAB†, Mediawiki, MutliMarkdown† |
| N | NAnt Build File†, Nginx, Nim, Ninja, Nix, NSIS |
| O | Objective-C†, Objective-C++†, OCaml†, OCamllex†, OCamlyacc†, Org Mode |
| P | Pascal†, Perl†, PHP†, PowerShell\*, Protobuf, Puppet, PureScript, Python† |
| Q | QML |
| R | R†, Racket, Rd†, Rego, Regular Expression†, Requirements.txt, reStructuredText†, Robot Framework, Ruby†, Ruby Haml†, Ruby on Rails†, Ruby Slim, Rust† |
| S | Sass\*, Scala†, SCSS, Salt State SLS\*, SML, Solidity, SQL†, Strace, Stylus, Svelte, Swift, SystemVerilog |
| T | Tcl†, Terraform, TeX†, Textile†, Todo.txt, TOML, TypeScript, TypescriptReact |
| V | Varlink, Verilog, VimL, Vue, Vyper |
| W | WGSL |
| X | XML† |
| Y | YAML† |
| Z | Zig |

## Themes

_Note: For visual examples of all of the embedded themes look at the docs for
[`two_face::theme::EmbeddedThemeName`](https://docs.rs/two-face/latest/two_face/theme/enum.EmbeddedThemeName.html)_

The full listing of themes provided by `two_face::theme`. Many of these themes
only make sense situationally, so you'll likely want to only expose a subset

- † Included in `syntect`'s bundled defaults

|  | Theme |
| :---: | :---: |
| 1 | 1337 (aka leet) |
| A | Ansi |
| B | Base16, Base16-256, Base16-Eighties (dark)†, Base16-Mocha (dark)†, Base16-Ocean (light/dark)† |
| C | Coldark (cold/dark aka light/dark) |
| D | DarkNeon, Dracula |
| G | GitHub, gruvbox (light/dark) |
| I | InspiredGitHub† |
| M | Monokai Extended (plain, bright, light, and origin) |
| N | Nord |
| O | One Half (light/dark) |
| S | Solarized (light/dark)†, Sublime Snazzy |
| T | TwoDark |
| V | Visual Studio Dark+ |
| Z | Zenburn |

## Legal

Most of the code for generating the syntax and theme dumps along with curating
said syntax and themes is taken from [`bat`](https://github.com/sharkdp/bat).
Because of this we also mirror `bat`'s licenses by being dual licensed under MIT
and Apache-2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files
for license details.

The embedded syntax definitions and assets also have their own licenses which
are compiled into
[this markdown file](https://github.com/CosmicHorrorDev/two-face/blob/main/generated/acknowledgements_full.md)
along with programmatic access in the `acknowledgement` module.

### `bat`'s NOTICE

Copyright (c) 2018-2021 bat-developers (https://github.com/sharkdp/bat).

bat is made available under the terms of either the MIT License or the Apache
License 2.0, at your option.

See the [LICENSE-APACHE](./bat/LICENSE-APACHE) and
[LICENSE-MIT](./bat/LICENSE-MIT) files for license details.
