use reqwest::header::AUTHORIZATION;
use reqwest::{Client, RequestBuilder};

use crate::config::SystemConfig;

pub struct TwitterClient {
    pub token: String,
    pub endpoint: String,
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

    pub async fn get_mentions(&self, user_id: u64) -> Result<String, reqwest::Error> {
        self.get(format!("/users/{}/mentions", user_id))
            .send()
            .await?
            .text()
            .await
    }

    pub async fn get_tweets(&self, user_id: u64) -> Result<String, reqwest::Error> {
        self.get(format!("/users/{}/tweets", user_id))
            .send()
            .await?
            .text()
            .await
    }
}
