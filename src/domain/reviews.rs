use std::sync::Arc;

use crate::client::twitter::{MentionsResponse, Tweet};
use crate::models::reviews::t::Source;
use crate::repository::reviews::{InsertError, Repository};

fn create_review(repo: Arc<dyn Repository>, tweet: &Tweet) {
    match repo.insert(&tweet.author_id, &tweet.id, Source::Twitter, &tweet.text) {
        Ok(()) => (),
        Err(InsertError::Duplicattion) => (),
    }
}

pub fn create_reviews(repo: Arc<dyn Repository>, resp: MentionsResponse) {
    match resp.data {
        Some(d) => d
            .iter()
            .for_each(|tweet| create_review(repo.clone(), tweet)),
        None => (),
    };
}
