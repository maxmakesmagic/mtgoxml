//! Tool to download and format the XML data files from Magic: the Gathering Online
#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use clickonce::deploymentmanifest::DeploymentManifest;

/// Errors returned by this library
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// HTTP error
    #[error("HTTP error")]
    HTTP(#[from] reqwest::Error),

    /// XML error
    #[error("XML error")]
    XML(#[from] serde_xml_rs::Error),

    /// Generic error
    #[error("Generic error {0}")]
    Generic(String),
}

/// Structure containing parsed information from Magic: the Gathering Online's ClickOnce application
#[derive(Debug)]
pub struct MtgoApplication {
    #[allow(dead_code)]
    deployment_manifest: DeploymentManifest,
}

impl MtgoApplication {
    /// Get the application from the default URL
    pub async fn default() -> Result<Self, Error> {
        Self::from_url("http://mtgoclientdepot.onlinegaming.wizards.com/MTGO.application").await
    }

    /// Get the application from a custom URL.
    pub async fn from_url(url: &str) -> Result<Self, Error> {
        let contents = reqwest::get(url).await?.text().await?;
        Self::from_contents(&contents)
    }

    /// Get the application from file contents.
    pub fn from_contents(contents: &str) -> Result<Self, Error> {
        // Parse the contents after trimming any BOM marker
        let deployment_manifest = serde_xml_rs::from_str(contents.trim_start_matches('\u{feff}'))?;
        Ok(Self {
            deployment_manifest,
        })
    }
}
