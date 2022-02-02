mod client;
mod config;

use client::twitter::TwitterClient;
use config::{SystemConfig, SYSTEM_CONFIG};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system_config_instance = SystemConfig::new();
    SYSTEM_CONFIG.set(system_config_instance).unwrap();

    let system_config = SystemConfig::global();

    let twitter_client = TwitterClient::new(&system_config.twitter.token);

    let resp = twitter_client
        .get_mentions(system_config.twitter.official_account_id, 100)
        .await?;
    println!("{:#?}", resp);

    //let resp = twitter_client
    //.get_tweets(system_config.twitter.official_account_id)
    //.await?;
    //println!("{:#?}", resp);

    //let resp = twitter_client
    //.update_status(1364619242335727616, String::from("Cut the SHIT!"))
    //.await?;
    //println!("{:#?}", resp);
    Ok(())
}
