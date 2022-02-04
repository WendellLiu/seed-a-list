use std::panic::panic_any;

use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;

use crate::db::pool::MysqlPool;
use crate::models::reviews::t::{NewReview, Source};
use crate::schema::reviews::dsl::reviews;

#[derive(Debug)]
pub enum InsertError {
    Duplicattion,
}

pub trait Repository: Send + Sync {
    fn insert(
        &self,
        external_author_id: &String,
        external_id: &String,
        source: Source,
        content: &String,
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
    ) -> Result<(), InsertError> {
        let pool = &self.pool;
        let conn = pool.get().unwrap();
        let new_post = NewReview {
            external_id,
            external_author_id,
            source,
            content: Some(content),
        };

        match diesel::insert_into(reviews)
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
