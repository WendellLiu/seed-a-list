mod client;
mod config;

use client::twitter::TwitterClient;
use config::{SystemConfig, SYSTEM_CONFIG};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system_config_instance = SystemConfig::new();
    SYSTEM_CONFIG.set(system_config_instance).unwrap();

    let system_config = SystemConfig::global();

    let twitter_client = TwitterClient {
        token: system_config.twitter.token.clone(),
    };
    let resp = twitter_client.get_mentions().await?;

    println!("{:#?}", resp);
    Ok(())
}
