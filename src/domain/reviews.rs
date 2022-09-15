use std::sync::Arc;

use regex::Regex;

use crate::client::twitter::{MentionsResponse, Tweet};
use crate::models::reviews::t::Source;
use crate::repository::reviews::{InsertError, Repository, ReviewWithTags};

fn parse_tags(content: &String) -> Vec<String> {
    let re = Regex::new(r"(#(?P<hashtag>[\w-]+))").unwrap();
    let mut hashtags = Vec::new();
    for caps in re.captures_iter(content) {
        hashtags.push(String::from(&caps["hashtag"]));
    }

    hashtags
}

pub enum CreateReviewError {
    DBError,
    InvalidResponse,
}

fn create_review(repo: Arc<dyn Repository>, tweet: &Tweet) -> Result<(), CreateReviewError> {
    let tags = parse_tags(&tweet.text);

    let review_with_tags = ReviewWithTags {
        external_author_id: tweet.author_id.clone(),
        external_id: tweet.id.clone(),
        content: tweet.text.clone(),
        tags,
    };

    match repo.insert(review_with_tags, Source::Twitter) {
        Ok(_count) => Ok(()),
        Err(InsertError::Duplication) => Err(CreateReviewError::DBError),
        Err(InsertError::Transaction) => Err(CreateReviewError::DBError),
    }
}

pub fn create_reviews(
    repo: Arc<dyn Repository>,
    resp: MentionsResponse,
) -> Result<(), CreateReviewError> {
    let resp = match resp.data {
        Some(tweets) => {
            let review_with_tags_list = tweets
                .into_iter()
                .map(|tweet| {
                    let tags = parse_tags(&tweet.text);

                    ReviewWithTags {
                        external_author_id: tweet.author_id,
                        external_id: tweet.id,
                        content: tweet.text,
                        tags,
                    }
                })
                .collect();

            repo.insert_multi(review_with_tags_list, Source::Twitter)
        }
        None => return Err(CreateReviewError::InvalidResponse),
    };

    match resp {
        Ok(_) => Ok(()),
        Err(InsertError::Duplication) => Err(CreateReviewError::DBError),
        Err(InsertError::Transaction) => Err(CreateReviewError::DBError),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_parse_tags() {
        let tags = parse_tags(&String::from(
            "https://example.com/music1 \n it's a great song! #happy and #upset \n\n#CityPop",
        ));
        assert_eq!(tags, vec!["happy", "upset", "CityPop"]);
    }
}
