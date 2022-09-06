use std::sync::Arc;

use regex::Regex;

use crate::client::twitter::{MentionsResponse, Tweet};
use crate::models::reviews::t::Source;
use crate::repository::reviews::{InsertError, Repository};

fn parse_tags(content: &String) -> Vec<String> {
    let re = Regex::new(r"(#(?P<hashtag>[a-z\d-]+))").unwrap();
    let mut hashtags = Vec::new();
    for caps in re.captures_iter(content) {
        hashtags.push(String::from(&caps["hashtag"]));
    }

    hashtags
}

fn create_review(repo: Arc<dyn Repository>, tweet: &Tweet) {
    let tags = parse_tags(&tweet.text);

    match repo.insert(
        &tweet.author_id,
        &tweet.id,
        Source::Twitter,
        &tweet.text,
        &tags,
    ) {
        Ok(()) => (),
        Err(InsertError::Duplicattion) => (),
        Err(InsertError::Transaction) => (),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_parse_tags() {
        let tags = parse_tags(&String::from(
            "https://example.com/music1 \n it's a great song! #happy and #upset",
        ));
        assert_eq!(tags, vec!["happy", "upset"]);
    }
}
