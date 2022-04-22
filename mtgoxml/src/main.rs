//! Tool to download and format the XML data files from Magic: the Gathering Online

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct DependentAssembly {
    codebase: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Dependency {
    #[serde(alias = "dependentAssembly")]
    assembly: DependentAssembly,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct AssemblyIdentity {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Assembly {
    #[serde(alias = "assemblyIdentity")]
    identity: AssemblyIdentity,
    dependency: Dependency,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let url = "http://mtgoclientdepot.onlinegaming.wizards.com/MTGO.application";

    let body = reqwest::get(url).await?.text().await?;

    let assembly: Assembly = serde_xml_rs::from_str(body.trim_start_matches('\u{feff}'))?;

    println!("Application is {:?}", assembly);

    Ok(())
}
