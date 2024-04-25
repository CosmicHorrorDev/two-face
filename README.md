# _two-face_

[![build status](https://img.shields.io/github/actions/workflow/status/CosmicHorrorDev/two-face/CI.yml?branch=main)](https://github.com/CosmicHorrorDev/two-face/actions)
[![Crates.io](https://img.shields.io/crates/v/two-face.svg)](https://crates.io/crates/two-face)
[![codecov](https://codecov.io/gh/CosmicHorrorDev/two-face/graph/badge.svg?token=MUORSBCHF2)](https://codecov.io/gh/CosmicHorrorDev/two-face)


Dedicated to chasing the [`bat` man](https://github.com/sharkdp)

Extra syntax and theme definitions for
[`syntect`](https://docs.rs/syntect/latest/syntect/) including many common ones
that are missing from the default set like TOML, TypeScript, and Dockerfile

## Example

The following

```cmd
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

Some embedded syntaxes use features that aren't available with `fancy-regex`. To
keep regex compilation infallible it's important to match this library's regex
implementation with the one you're using from syntect

To use [`Oniguruma`](https://github.com/kkos/oniguruma) aka onig

```cmd
[dependencies]
# `onig` is the default
syntect = "0.5.1"
two-face = "0.3.1"
```

To use [`fancy-regex`](https://github.com/fancy-regex/fancy-regex)

```toml
[dependencies]
syntect = { version = "0.5.1", default-features = false, features = ["default-fancy"] }
two-face = { version = "0.3.1", default-features = false, features = ["syntect-fancy"] }
```

## Legal

Most of the code for generating the syntax and theme dumps along with curating
said syntax and themes is taken from [`bat`](https://github.com/sharkdp/bat).
Because of this we also mirror `bat`'s licenses by being dual licensed under MIT
and Apache-2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files
for license details.

### `bat`'s NOTICE

Copyright (c) 2018-2021 bat-developers (https://github.com/sharkdp/bat).

bat is made available under the terms of either the MIT License or the Apache
License 2.0, at your option.

See the LICENSE-APACHE and LICENSE-MIT files for license details.
