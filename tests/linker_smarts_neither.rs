mod utils;

#[test]
fn asset_check() {
    utils::linker_strips_all_but(std::iter::empty());
}
