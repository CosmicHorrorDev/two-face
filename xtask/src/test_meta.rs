use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::utils;

use cargo_lock::{Lockfile, Version};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    version: Version,
    assets: AssetMeta,
}

pub fn update_test_metadata() {
    let version = syntect_version();
    let source = locate_syntect_source(&version);
    let assets = AssetMeta::from_syntect_source(&source);
    let meta = Meta { version, assets };
    let header_comment = r"# !! generated content: Update by running `$ cargo xtask test-meta` !!";
    let meta_toml = toml::to_string(&meta).unwrap();
    let out_path = Path::new("tests").join("assets").join("syntect-meta.toml");
    log::info!("Storing new syntect metadata at: {}", out_path.display());
    fs::write(&out_path, format!("{header_comment}\n\n{meta_toml}")).unwrap();
}

fn syntect_version() -> Version {
    let lockfile = Lockfile::load("Cargo.lock").unwrap();
    let syntect_versions: Vec<_> = lockfile
        .packages
        .into_iter()
        .filter_map(|p| (p.name.as_str() == "syntect").then_some(p.version))
        .collect();
    match syntect_versions.as_slice() {
        [one] => one.to_owned(),
        [] => panic!("`syntect` wasn't found in the lockfile"),
        two_or_more => panic!("Found multiple distinct `syntect` versions: {two_or_more:?}"),
    }
}

fn locate_syntect_source(version: &Version) -> PathBuf {
    let syntect_dir = format!("syntect-{version}");
    let cargo_home = home::cargo_home().unwrap();
    let src_dir = cargo_home.join("registry").join("src");
    fs::read_dir(&src_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().join(&syntect_dir))
        .find(|p| p.is_dir())
        .expect("Unable to locate source")
}

#[derive(Debug, Deserialize, Serialize)]
struct AssetMeta {
    syn_newlines: utils::AssetFingerprint,
    syn_no_newlines: utils::AssetFingerprint,
    themes: utils::AssetFingerprint,
}

impl AssetMeta {
    fn from_syntect_source(src: &Path) -> Self {
        let assets_dir = src.join("assets");
        let syn_newlines =
            utils::AssetFingerprint::from_path(20, &assets_dir.join("default_newlines.packdump"))
                .unwrap();
        let syn_no_newlines =
            utils::AssetFingerprint::from_path(20, &assets_dir.join("default_nonewlines.packdump"))
                .unwrap();
        let themes =
            utils::AssetFingerprint::from_path(10, &assets_dir.join("default.themedump")).unwrap();
        Self {
            syn_newlines,
            syn_no_newlines,
            themes,
        }
    }
}
