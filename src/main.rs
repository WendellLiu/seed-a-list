mod client;
mod config;

use client::twitter::foo;
use config::{SystemConfig, SYSTEM_CONFIG};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let system_config_instance = SystemConfig::new();
    SYSTEM_CONFIG.set(system_config_instance).unwrap();

    let system_config = SystemConfig::global();

    println!("system config: {}", system_config.twitter.token);

    let resp = foo().await?;

    println!("{:#?}", resp);
    Ok(())
}
