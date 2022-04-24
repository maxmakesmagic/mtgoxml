//! A library for parsing ClickOnce applications
#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use serde::Deserialize;

/// Trait defined for all structures that can be deserialized from
/// XML.
pub trait FromXML<'de> {
    /// Converts an XML string into a structure, or returns an error.
    fn from_xml(contents: &str) -> Result<Self, serde_xml_rs::Error>
    where
        Self: Sized + Deserialize<'de>,
    {
        serde_xml_rs::from_str(contents)
    }
}

pub mod deploymentmanifest;
