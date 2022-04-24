//! Tool to download and format the XML data files from Magic: the Gathering Online

use log::info;
use mtgoxml::MtgoApplication;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let mtgo = MtgoApplication::default().await?;
    info!("Application is {:#?}", mtgo);

    Ok(())
}
