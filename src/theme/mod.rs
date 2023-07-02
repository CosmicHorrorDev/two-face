// TODO: have an enum to enumerate themes? This allows for nicer programmatic access

pub use self::core_types::LazyThemeSet;

mod core_types;

pub fn extra() -> LazyThemeSet {
    syntect::dumps::from_binary(include_bytes!("../../generated/themes.bin"))
}
