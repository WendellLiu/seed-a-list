use reqwest::header::AUTHORIZATION;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::config::SystemConfig;

pub struct TwitterClient {
    pub token: String,
    pub endpoint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterMeta {
    pub oldest_id: String,
    pub newest_id: String,
    pub result_count: u32,
    pub next_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MentionsResponse {
    pub data: Vec<Tweet>,
    pub meta: TwitterMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetsResponse {
    pub data: Vec<Tweet>,
    pub meta: TwitterMeta,
}

impl TwitterClient {
    pub fn new(token: &String) -> TwitterClient {
        let system_config = SystemConfig::global();

        TwitterClient {
            token: token.clone(),
            endpoint: system_config.twitter.endpoint.clone(),
        }
    }

    fn get(&self, namespace: String) -> RequestBuilder {
        let client = Client::new();
        let token = format!("Bearer {}", self.token);
        let url = format!("{}{}", self.endpoint, namespace);
        client.get(&url).header(AUTHORIZATION, token)
    }

    pub async fn get_mentions(&self, user_id: u64) -> Result<MentionsResponse, reqwest::Error> {
        self.get(format!("/users/{}/mentions", user_id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_tweets(&self, user_id: u64) -> Result<TweetsResponse, reqwest::Error> {
        self.get(format!("/users/{}/tweets", user_id))
            .send()
            .await?
            .json()
            .await
    }
}
