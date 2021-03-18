use std::future::Future;

use reqwest::Client;

pub async fn foo() -> Result<String, reqwest::Error> {
    let client = Client::new();

    client
        .get("https://api.twitter.com/2/users/4781015496/mentions")
        .send()
        .await?
        .text()
        .await
}
