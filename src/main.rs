mod client;
mod config;
mod db;
mod domain;
mod models;
mod repository;
mod schema;

#[macro_use]
extern crate diesel;

use std::sync::Arc;

use client::twitter::TwitterClient;
use config::{SystemConfig, SYSTEM_CONFIG};
use db::pool::establish_pool;
use domain::reviews::create_reviews;
use repository::reviews::MysqlRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system_config_instance = SystemConfig::new("config.yml");
    SYSTEM_CONFIG.set(system_config_instance).unwrap();

    let system_config = SystemConfig::global();

    println!("{:#?}", &system_config);
    let twitter_client = TwitterClient::new(&system_config.twitter.token);

    let resp = twitter_client
        .get_mentions(system_config.twitter.official_account_id, 100)
        .await?;
    println!("{:#?}", resp);

    let pool = establish_pool(&system_config.mysql.endpoint);
    let repo = MysqlRepository { pool };
    //create_reviews(Arc::new(repo), resp);

    Ok(())
}
