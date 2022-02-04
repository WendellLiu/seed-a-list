mod client;
mod config;
mod db;
mod models;
mod schema;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;

use client::twitter::TwitterClient;
use config::{SystemConfig, SYSTEM_CONFIG};
use db::pool::establish_pool;
use models::reviews::NewReview;

pub fn create_review(
    conn: &MysqlConnection,
    external_author_id: &String,
    external_id: &String,
    source: &String,
    content: &String,
) {
    use schema::reviews::dsl::reviews;

    let new_post = NewReview {
        external_id,
        external_author_id,
        source,
        content: Some(content),
    };

    diesel::insert_into(reviews)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
}

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

    let pool = establish_pool(&system_config.mysql.endpoint);

    match resp.data {
        Some(d) => d.iter().for_each(|tweet| {
            create_review(
                &pool.get().unwrap(),
                &tweet.author_id,
                &tweet.id,
                &String::from("twitter"),
                &tweet.text,
            );
        }),
        None => (),
    };

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
