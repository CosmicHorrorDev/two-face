// Another case where we want to share code with the main crate, but don't want to create another
// dependency that we have to publish (even just a dev-dependency)
include!("../../tests/utils/mod.rs");

use std::{io, path::Path};

impl AssetFingerprint {
    pub fn from_path(prefix_len: usize, path: &Path) -> io::Result<Self> {
        let bytes = fs::read(path)?;
        Ok(Self::new(prefix_len, &bytes))
    }

    pub fn new(prefix_len: usize, bytes: &[u8]) -> Self {
        let size = bytes.len();
        let hash = xxhash(bytes);
        let prefix = bytes[..prefix_len.min(size)].to_vec();
        Self { prefix, hash, size }
    }
}
