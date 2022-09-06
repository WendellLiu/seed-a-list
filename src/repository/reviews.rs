use std::panic::panic_any;

use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error;

use crate::db::pool::MysqlPool;
use crate::models::review_tags::t::NewReviewTag;
use crate::models::reviews::t::{NewReview, Source};
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
    ) -> Result<(), InsertError>;
}

pub struct MysqlRepository {
    pub pool: MysqlPool,
}

impl Repository for MysqlRepository {
    fn insert(
        &self,
        external_author_id: &String,
        external_id: &String,
        source: Source,
        content: &String,
        tags: &Vec<String>,
    ) -> Result<(), InsertError> {
        let pool = &self.pool;
        let conn = pool.get().unwrap();

        conn.transaction(|| {
            let new_review = NewReview {
                external_id,
                external_author_id,
                source,
                content: Some(content),
            };

            //let new_reveiw_tags =

            diesel::insert_into(reviews)
                .values(&new_post)
                .execute(&conn)
                .map(|_| ());

            diesel::insert_into(review_tags)
                .values(&new_review)
                .execute(&conn)
                .map(|_| ())
        })
        .map_err(|e| InsertError::from(e))
    }
}
