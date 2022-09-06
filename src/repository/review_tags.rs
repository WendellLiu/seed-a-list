use std::panic::panic_any;

use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;

use crate::db::pool::MysqlPool;
use crate::models::review_tags::t::NewReviewTag;
use crate::schema::review_tags::dsl::review_tags;

#[derive(Debug)]
pub enum InsertError {
    Duplicattion,
}

pub trait Repository: Send + Sync {
    fn insert(&self, review_id: i32, name: &String) -> Result<(), InsertError>;
}

pub struct MysqlRepository {
    pub pool: MysqlPool,
}

impl Repository for MysqlRepository {
    fn insert(&self, review_id: i32, name: &String) -> Result<(), InsertError> {
        let pool = &self.pool;
        let conn = pool.get().unwrap();
        let new_post = NewReviewTag { review_id, name };

        match diesel::insert_into(review_tags)
            .values(&new_post)
            .execute(&conn)
        {
            Ok(_affected_rows) => Ok(()),
            Err(DatabaseError(UniqueViolation, info)) => {
                println!("{}", info.message());
                Err(InsertError::Duplicattion)
            }
            Err(e) => panic_any(e),
        }
    }
}
