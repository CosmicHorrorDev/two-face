use std::{env, fs, hash::Hasher};

use object::{Object, ObjectSection};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use twox_hash::XxHash64;

fn u32_from_last_half_u64(uint: u64) -> u32 {
    let [_, _, _, _, b1, b2, b3, b4] = uint.to_le_bytes();
    u32::from_le_bytes([b1, b2, b3, b4])
}

fn xxhash(bytes: &[u8]) -> u32 {
    let mut hasher = XxHash64::default();
    hasher.write(bytes);
    let uint = hasher.finish();
    u32_from_last_half_u64(uint)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asset {
    Syntect(SyntectAsset),
    TwoFace(TwoFaceAsset),
}

impl Asset {
    pub fn iter() -> impl Iterator<Item = Asset> {
        SyntectAsset::iter()
            .map(Into::into)
            .chain(TwoFaceAsset::iter().map(Into::into))
    }
}

impl From<TwoFaceAsset> for Asset {
    fn from(v: TwoFaceAsset) -> Self {
        Self::TwoFace(v)
    }
}

impl From<SyntectAsset> for Asset {
    fn from(v: SyntectAsset) -> Self {
        Self::Syntect(v)
    }
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
pub enum TwoFaceAsset {
    AckFull,
    SynOnigNewlines,
    SynFancyNewlines,
    SynOnigNoNewlines,
    SynFancyNoNewlines,
    Themes,
}

impl TwoFaceAsset {
    pub fn rel_path(self) -> &'static str {
        match self {
            Self::AckFull => "generated/acknowledgements_full.bin",
            Self::SynOnigNewlines => "generated/syntaxes-onig-newlines.bin",
            Self::SynFancyNewlines => "generated/syntaxes-fancy-newlines.bin",
            Self::SynOnigNoNewlines => "generated/syntaxes-onig-no-newlines.bin",
            Self::SynFancyNoNewlines => "generated/syntaxes-fancy-no-newlines.bin",
            Self::Themes => "generated/themes.bin",
        }
    }

    pub fn contents(self) -> Vec<u8> {
        fs::read(self.rel_path()).unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SyntectMeta {
    pub version: cargo_lock::Version,
    assets: SyntectAssetMeta,
}

impl SyntectMeta {
    pub fn load() -> Self {
        let contents = include_str!("../assets/syntect-meta.toml");
        toml::from_str(contents).unwrap()
    }

    fn get(&self, asset: SyntectAsset) -> &AssetFingerprint {
        match asset {
            SyntectAsset::SynNewlines => &self.assets.syn_newlines,
            SyntectAsset::SynNoNewlines => &self.assets.syn_no_newlines,
            SyntectAsset::Themes => &self.assets.themes,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct SyntectAssetMeta {
    syn_newlines: AssetFingerprint,
    syn_no_newlines: AssetFingerprint,
    themes: AssetFingerprint,
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
pub enum SyntectAsset {
    SynNewlines,
    SynNoNewlines,
    Themes,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AssetFingerprint {
    prefix: Vec<u8>,
    hash: u32,
    size: usize,
}

impl AssetFingerprint {
    pub fn is_found_within(&self, bytes: &[u8]) -> bool {
        let Self { prefix, hash, size } = self;
        let upper_bound = bytes.len().saturating_sub(prefix.len());
        for start_idx in 0..upper_bound {
            let maybe_prefix = &bytes[start_idx..][..prefix.len()];
            if self.prefix == maybe_prefix {
                let Some(maybe_asset) = &bytes[start_idx..].get(..*size) else {
                    return false;
                };
                let maybe_hash = xxhash(maybe_asset);
                if *hash == maybe_hash {
                    return true;
                }
            }
        }

        false
    }
}

impl From<Asset> for AssetFingerprint {
    fn from(asset: Asset) -> Self {
        match asset {
            Asset::Syntect(a) => a.into(),
            Asset::TwoFace(a) => a.into(),
        }
    }
}

impl From<TwoFaceAsset> for AssetFingerprint {
    fn from(asset: TwoFaceAsset) -> Self {
        // Our "prefix" is just the whole thing because we can
        let prefix = asset.contents();
        let hash = xxhash(&prefix);
        let size = prefix.len();
        Self { prefix, hash, size }
    }
}

impl From<SyntectAsset> for AssetFingerprint {
    fn from(asset: SyntectAsset) -> Self {
        SyntectMeta::load().get(asset).to_owned()
    }
}

// Sharing code between different sets of integration tests aint easy to do cleanly
#[allow(dead_code)]
pub fn linker_strips_all_but(assets: impl Iterator<Item = Asset>) {
    let expected_assets: Vec<_> = assets.collect();
    let asset_to_expected = Asset::iter().map(|asset| {
        let expected = expected_assets.contains(&asset);
        (asset, expected)
    });
    let this_bin = env::args().next().unwrap();
    let bin_contents = fs::read(&this_bin).unwrap();

    let obj_file = object::File::parse(bin_contents.as_slice()).unwrap();
    let rodata_section = obj_file.section_by_name(".rodata").unwrap();
    let rodata = rodata_section.uncompressed_data().unwrap();

    for (asset, expected) in asset_to_expected {
        let fingerprint: AssetFingerprint = asset.into();
        if expected {
            assert!(
                fingerprint.is_found_within(&rodata),
                "Failed locating {asset:?} in R/O data"
            );
        } else {
            assert!(
                !fingerprint.is_found_within(&rodata),
                "Located unexpected asset {asset:?} in R/O data"
            );
        }
    }
}
