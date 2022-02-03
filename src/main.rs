mod client;
mod config;
mod models;
mod schema;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;

use client::twitter::TwitterClient;
use config::{SystemConfig, SYSTEM_CONFIG};

use crate::models::reviews::{NewReview, Review};

pub fn establish_connection(database_url: &String) -> MysqlConnection {
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

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

    let mysql_conn = establish_connection(&system_config.mysql.endpoint);

    match resp.data {
        Some(d) => d.iter().for_each(|tweet| {
            create_review(
                &mysql_conn,
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
