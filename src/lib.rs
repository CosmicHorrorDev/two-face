#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg_attr(docsrs, doc(cfg(feature = "acknowledgement")))]
#[cfg(feature = "acknowledgement")]
pub mod acknowledgement;
#[cfg_attr(docsrs, doc(cfg(feature = "extra-syntax")))]
#[cfg(feature = "extra-syntax")]
pub mod syntax;
#[cfg_attr(docsrs, doc(cfg(feature = "extra-theme")))]
#[cfg(feature = "extra-theme")]
pub mod theme;

pub fn acknowledgement_url() -> String {
    todo!("Add a path here pointing to the repo file once this is ready to publish");
}

// TODO: add more extensive tests later
#[cfg(test)]
mod tests {
    // The serialized data is in the right structure
    #[test]
    fn sanity() {
        #[cfg(feature = "acknowledgement")]
        super::acknowledgement::listing();
        #[cfg(feature = "extra-syntax")]
        super::syntax::extra();
        #[cfg(feature = "extra-theme")]
        super::theme::extra();
    }
}
