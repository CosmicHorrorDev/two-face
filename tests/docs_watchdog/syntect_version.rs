use cargo_lock::{Lockfile, Version};

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
    // !!IMPORTANT!!: if you change the version number here then make sure you also update all of
    // the relevant values for `syntect`'s assets under `tests/utils`
    assert_eq!(syntect_version, &Version::new(5, 2, 0));
}
