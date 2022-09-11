pub mod t {
    use crate::schema::reviews;

    use chrono::NaiveDateTime;
    use diesel::backend::Backend;
    use diesel::serialize::{self, Output, ToSql};
    use diesel::sql_types::VarChar;

    #[derive(Debug, Copy, Clone, AsExpression, FromSqlRow)]
    #[diesel(sql_type = VarChar)]
    pub enum Source {
        Twitter,
    }

    impl<DB: Backend> ToSql<VarChar, DB> for Source
    where
        str: ToSql<VarChar, DB>,
    {
        fn to_sql<'b>(&self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
            let v: &str = match *self {
                Source::Twitter => "Twitter",
            };
            (v as &str).to_sql(out)
        }
    }

    #[derive(Queryable)]
    pub struct Review {
        pub id: i32,
        pub external_author_id: String,
        pub external_id: String,
        pub source: String,
        pub content: Option<String>,
        pub created_at: Option<NaiveDateTime>,
        pub updated_at: Option<NaiveDateTime>,
    }

    #[derive(Insertable)]
    #[diesel(table_name = reviews)]
    pub struct NewReview<'a> {
        pub external_author_id: &'a str,
        pub external_id: &'a str,
        pub source: Source,
        pub content: Option<&'a str>,
    }
}
