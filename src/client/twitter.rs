use reqwest::header::AUTHORIZATION;
use reqwest::{Client, IntoUrl, RequestBuilder};

pub struct TwitterClient {
    pub token: String,
}

impl TwitterClient {
    fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        let client = Client::new();
        let token = format!("Bearer {}", self.token);
        client.get(url).header(AUTHORIZATION, token)
    }

    pub async fn get_mentions(&self) -> Result<String, reqwest::Error> {
        self.get("https://api.twitter.com/2/users/4781015496/mentions")
            .send()
            .await?
            .text()
            .await
    }
}
