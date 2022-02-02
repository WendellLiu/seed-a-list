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

    //fn post<Body: Serialize>(&self, namespace: String, body: Body) -> RequestBuilder {
    //let client = Client::new();
    //let token = format!("Bearer {}", self.token);
    //let url = format!("{}{}", self.endpoint, namespace);
    //client.post(&url).header(AUTHORIZATION, token).json(&body)
    //}

    pub async fn get_mentions(
        &self,
        user_id: u64,
        max_results: u8,
    ) -> Result<MentionsResponse, reqwest::Error> {
        self.get(format!("/2/users/{}/mentions", user_id))
            .query(&[("max_results", max_results)])
            .send()
            .await?
            .json()
            .await
    }
}
