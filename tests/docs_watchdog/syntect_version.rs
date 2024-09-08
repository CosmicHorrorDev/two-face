use crate::utils;

use cargo_lock::Lockfile;

#[test]
fn locked_version() {
    let lockfile = Lockfile::load("Cargo.lock").unwrap();
    let syntect_versions: Vec<_> = lockfile
        .packages
        .into_iter()
        .filter_map(|p| (p.name.as_str() == "syntect").then_some(p.version))
        .collect();
    let syntect_version = match syntect_versions.as_slice() {
        [one] => one,
        [] => panic!("`syntect` wasn't found in the lockfile"),
        two_or_more => panic!("Found multiple distinct `syntect` versions: {two_or_more:?}"),
    };

    let meta = utils::SyntectMeta::load();
    assert_eq!(
        syntect_version, &meta.version,
        "If this fails then run `$ cargo xtask test-meta` to refresh the metadata"
    );
}
