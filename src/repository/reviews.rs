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
    Duplicattion,
    Transaction,
}

impl From<Error> for InsertError {
    fn from(e: Error) -> Self {
        match e {
            Error::DatabaseError(UniqueViolation, info) => {
                println!("{}", info.message());
                InsertError::Duplicattion
            }
            Error::RollbackTransaction => InsertError::Transaction,
            e => panic_any(e),
        }
    }
}

pub trait Repository: Send + Sync {
    fn insert(&self, review_tags: ReviewWithTags) -> Result<usize, InsertError>;
}

pub struct MysqlRepository {
    pub pool: MysqlPool,
}

pub struct ReviewWithTags<'a> {
    pub external_author_id: &'a str,
    pub external_id: &'a str,
    pub source: Source,
    pub content: &'a str,
    pub tags: Vec<String>,
}

impl Repository for MysqlRepository {
    fn insert(&self, review_with_tags: ReviewWithTags) -> Result<usize, InsertError> {
        use crate::schema::reviews::dsl::{external_id, source};

        let pool = &self.pool;
        let mut conn = pool.get().unwrap();

        conn.transaction(|_| {
            let new_review = NewReview {
                external_id: &review_with_tags.external_id,
                external_author_id: &review_with_tags.external_author_id,
                source: review_with_tags.source,
                content: Some(&review_with_tags.content),
            };

            let mut conn2 = pool.get().unwrap();
            diesel::insert_into(reviews)
                .values(&new_review)
                .execute(&mut conn2)
                .map_err(|e| InsertError::from(e));

            let new_review: Review = reviews
                .filter(external_id.eq(review_with_tags.external_id))
                .filter(source.eq(review_with_tags.source))
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

    //fn insert_multi(&self) -> Result<usize, InsertError> {}
}
