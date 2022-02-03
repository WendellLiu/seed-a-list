use crate::schema::reviews;

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
    pub source: &'a str,
    pub content: Option<&'a str>,
}
