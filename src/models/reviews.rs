use crate::schema::reviews;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::VarChar;
use std::io;

#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "VarChar"]
pub enum Source {
    Twitter,
}

impl<DB: Backend> ToSql<VarChar, DB> for Source
where
    str: ToSql<VarChar, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        let v: &str = match *self {
            Source::Twitter => "Twitter",
        };
        (v as &str).to_sql(out)
    }
}

#[derive(Queryable)]
pub struct Review<'a> {
    pub id: i32,
    pub external_author_id: &'a str,
    pub external_id: &'a str,
    pub source: &'a str,
    pub content: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name = "reviews"]
pub struct NewReview<'a> {
    pub external_author_id: &'a str,
    pub external_id: &'a str,
    pub source: Source,
    pub content: Option<&'a str>,
}
