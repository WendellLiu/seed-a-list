use std::collections::HashMap;
use std::convert::identity;
use std::panic::panic_any;

use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error;

use crate::db::pool::MysqlPool;
use crate::models::review_tags::t::NewReviewTag;
use crate::models::reviews::t::{NewReview, Review, Source};
use crate::schema::review_tags::dsl::review_tags;
use crate::schema::reviews::dsl::reviews;

#[derive(Debug)]
pub enum InsertError {
    Duplication,
    Transaction,
}

impl From<Error> for InsertError {
    fn from(e: Error) -> Self {
        match e {
            Error::DatabaseError(UniqueViolation, info) => {
                println!("{}", info.message());
                InsertError::Duplication
            }
            Error::RollbackTransaction => InsertError::Transaction,
            e => panic_any(e),
        }
    }
}

pub struct ReviewWithTags {
    pub external_author_id: String,
    pub external_id: String,
    pub content: String,
    pub tags: Vec<String>,
}

pub trait Repository: Send + Sync {
    fn insert(
        &self,
        review_with_tags: ReviewWithTags,
        source: Source,
    ) -> Result<usize, InsertError>;
    fn insert_multi(
        &self,
        review_with_tags_list: Vec<ReviewWithTags>,
        source: Source,
    ) -> Result<usize, InsertError>;
}

pub struct MysqlRepository {
    pub pool: MysqlPool,
}

impl Repository for MysqlRepository {
    fn insert(
        &self,
        review_with_tags: ReviewWithTags,
        source: Source,
    ) -> Result<usize, InsertError> {
        use crate::schema::reviews::dsl as review_dsl;

        let pool = &self.pool;
        let mut conn = pool.get().unwrap();

        conn.transaction(|_| {
            let new_review = NewReview {
                external_id: &review_with_tags.external_id,
                external_author_id: &review_with_tags.external_author_id,
                source,
                content: Some(&review_with_tags.content),
            };

            let mut conn2 = pool.get().unwrap();
            diesel::insert_into(reviews)
                .values(&new_review)
                .execute(&mut conn2)
                .map_err(|e| InsertError::from(e))?;

            let new_review: Review = reviews
                .filter(review_dsl::external_id.eq(review_with_tags.external_id))
                .filter(review_dsl::source.eq(source))
                .first(&mut conn2)
                .unwrap();

            let new_reviwe_tags: Vec<NewReviewTag> = review_with_tags
                .tags
                .into_iter()
                .map(|tag| NewReviewTag {
                    review_id: new_review.id,
                    name: tag,
                })
                .collect();

            diesel::insert_into(review_tags)
                .values(new_reviwe_tags)
                .execute(&mut conn2)
                .map_err(|e| InsertError::from(e))
        })
        .map_err(|e| InsertError::from(e))
    }

    fn insert_multi(
        &self,
        review_with_tags_list: Vec<ReviewWithTags>,
        source: Source,
    ) -> Result<usize, InsertError> {
        use crate::schema::reviews::dsl as review_dsl;

        let pool = &self.pool;
        let mut conn = pool.get().unwrap();

        let new_reviews: Vec<NewReview> = review_with_tags_list
            .iter()
            .map(|review_with_tags| NewReview {
                external_id: &review_with_tags.external_id,
                external_author_id: &review_with_tags.external_author_id,
                source,
                content: Some(&review_with_tags.content),
            })
            .collect();

        let mut new_review_tags_tuple = vec![];

        for review_with_tags in review_with_tags_list.iter() {
            for tag in review_with_tags.tags.iter() {
                new_review_tags_tuple.push((review_with_tags.external_id.clone(), tag))
            }
        }

        conn.transaction(|_| {
            let mut conn2 = pool.get().unwrap();
            diesel::insert_into(reviews)
                .values(&new_reviews)
                .execute(&mut conn2)
                .map_err(|e| InsertError::from(e))?;

            let external_ids: Vec<String> = review_with_tags_list
                .iter()
                .map(|review_with_tags| review_with_tags.external_id.clone())
                .collect();

            let new_reviews: Vec<Review> = reviews
                .filter(review_dsl::external_id.eq_any(external_ids))
                .filter(review_dsl::source.eq(source))
                .load(&mut conn2)
                .unwrap();

            let mut external_id_to_id_map: HashMap<String, i32> = HashMap::new();

            for new_review in new_reviews.into_iter() {
                external_id_to_id_map
                    .entry(new_review.external_id)
                    .or_insert(new_review.id);
            }

            let new_reviwe_tags: Vec<NewReviewTag> = new_review_tags_tuple
                .into_iter()
                .map(|(external_id, tag)| {
                    let id = external_id_to_id_map.get(&external_id);

                    match id {
                        Some(review_id) => Some(NewReviewTag {
                            review_id: *review_id,
                            name: tag.to_string(),
                        }),
                        None => None,
                    }
                })
                .filter_map(identity)
                .collect();

            diesel::insert_into(review_tags)
                .values(new_reviwe_tags)
                .execute(&mut conn2)
                .map_err(|e| InsertError::from(e))
        })
        .map_err(|e| InsertError::from(e))
    }
}
