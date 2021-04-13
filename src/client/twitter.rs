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
    pub oldest_id: Option<String>,
    pub newest_id: Option<String>,
    pub result_count: u32,
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MentionsResponse {
    pub data: Option<Vec<Tweet>>,
    pub meta: TwitterMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetsResponse {
    pub data: Vec<Tweet>,
    pub meta: TwitterMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusUpdateRequest {
    pub in_reply_to_status_id: String,
    pub status: String,
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

    fn post<Body: Serialize>(&self, namespace: String, body: Body) -> RequestBuilder {
        let client = Client::new();
        let token = format!("Bearer {}", self.token);
        let url = format!("{}{}", self.endpoint, namespace);
        client.post(&url).header(AUTHORIZATION, token).json(&body)
    }

    pub async fn get_mentions(&self, user_id: u64) -> Result<MentionsResponse, reqwest::Error> {
        self.get(format!("/2/users/{}/mentions", user_id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_tweets(&self, user_id: u64) -> Result<TweetsResponse, reqwest::Error> {
        self.get(format!("/2/users/{}/tweets", user_id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn update_status(
        &self,
        tweet_id: u64,
        text: String,
    ) -> Result<String, reqwest::Error> {
        self.post(
            String::from("/1.1/statuses/update.json"),
            StatusUpdateRequest {
                in_reply_to_status_id: tweet_id.to_string(),
                status: text,
            },
        )
        .send()
        .await?
        .json()
        .await
    }
}
