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
    fn insert(
        &self,
        external_author_id: &String,
        external_id: &String,
        source: Source,
        content: &String,
        tags: &Vec<String>,
    ) -> Result<usize, InsertError>;
}

pub struct MysqlRepository {
    pub pool: MysqlPool,
}

impl Repository for MysqlRepository {
    fn insert(
        &self,
        external_author_id: &String,
        arg_external_id: &String,
        arg_source: Source,
        content: &String,
        tags: &Vec<String>,
    ) -> Result<usize, InsertError> {
        use crate::schema::reviews::dsl::{external_id, source};

        let pool = &self.pool;
        let mut conn = pool.get().unwrap();

        conn.transaction(|_| {
            let new_review = NewReview {
                external_id: arg_external_id,
                external_author_id,
                source: arg_source,
                content: Some(content),
            };

            let mut conn2 = pool.get().unwrap();
            diesel::insert_into(reviews)
                .values(&new_review)
                .execute(&mut conn2)
                .map_err(|e| InsertError::from(e));

            let new_review: Review = reviews
                .filter(external_id.eq(arg_external_id))
                .filter(source.eq(arg_source))
                .first(&mut conn2)
                .unwrap();

            let new_reviwe_tags: Vec<NewReviewTag> = tags
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
}
