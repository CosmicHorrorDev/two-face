// Could also handle this by having a core crate that this crate and `syntect-extra-defaults` use,
// but that seems excessive. Instead we can just add in the extra stuff used for generation here
include!("../../../src/acknowledgement/core_types.rs");

use std::{ffi::OsStr, fs, path::Path};

use super::utils;

use anyhow::Context;

pub const NORM_LICENSE_STEMS: &[&str] = &["copying", "notice", "license"];
const LICENSE_CACHE: &[u8] = include_bytes!("../../resources/spdx_cache.bin.zstd");
const SUBLIME_LICENSE: &str = include_str!("../../resources/sublime_packages_license");

impl License {
    fn new(store: &askalono::Store, base_dir: &Path, rel_path: &Path) -> anyhow::Result<Self> {
        let text = fs::read_to_string(base_dir.join(rel_path))?;
        let ty = LicenseType::new(store, &text)?;

        Ok(Self {
            ty,
            text,
            rel_path: rel_path.to_owned(),
        })
    }
}

impl LicenseType {
    fn new(store: &askalono::Store, text: &str) -> anyhow::Result<Self> {
        if text == SUBLIME_LICENSE {
            Ok(Self::Sublime)
        } else {
            let text_data = askalono::TextData::from(&*text);
            let askalono::Match { score, name, .. } = store.analyze(&text_data);
            anyhow::ensure!(score > 0.9, "Detection score is too low");
            let ty = match name {
                "MIT" => Self::Mit,
                "BSD-2-Clause" => Self::Bsd2Clause,
                "BSD-2-Clause-FreeBSD" => Self::Bsd2ClauseFreeBsd,
                "Unlicense" => Self::Unlicense,
                "BSD-3-Clause" => Self::Bsd3Clause,
                "Apache-2.0" => Self::Apache2,
                "WTFPL" => Self::Wtfpl,
                other => return Err(anyhow::anyhow!("Unrecognized license type: {other}")),
            };

            Ok(ty)
        }
    }
}

impl Acknowledgements {
    pub fn load_from_assets(assets_dir: &Path) -> anyhow::Result<Self> {
        let store = askalono::Store::from_cache(LICENSE_CACHE)?;
        let for_syntaxes = load_acknowledgements(&store, assets_dir, "syntaxes")?;
        let for_themes = load_acknowledgements(&store, assets_dir, "themes")?;
        Ok(Self {
            for_syntaxes,
            for_themes,
        })
    }
}

// TODO: fixup formatting
fn load_acknowledgements(
    store: &askalono::Store,
    base_dir: &Path,
    dir_name: &str,
) -> anyhow::Result<Vec<License>> {
    let mut licenses = Vec::new();

    // TODO: this loop is duped. Dedupe?
    let dir = base_dir.join(dir_name);
    for file in utils::walk_files(&dir)? {
        let is_license = file
            .file_stem()
            .and_then(OsStr::to_str)
            .map(|stem| NORM_LICENSE_STEMS.contains(&&*stem.to_ascii_lowercase()))
            .unwrap_or(false);
        if is_license {
            let rel_path = file
                .strip_prefix(base_dir)
                .expect("Entry should always be prefixed by base dir");
            let license = License::new(store, base_dir, &rel_path)
                .with_context(|| format!("Failed detecting license: {}", rel_path.display()))?;
            licenses.push(license);
        }
    }
    // TODO: need to normalize to handle filesystems with different case sensitivity

    Ok(licenses)
}
