use syntect::parsing::SyntaxSet;

pub fn extra() -> SyntaxSet {
    syntect::dumps::from_uncompressed_data(include_bytes!("../generated/syntaxes.bin")).unwrap()
}
