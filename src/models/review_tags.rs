pub mod t {
    use crate::schema::review_tags;

    #[derive(Queryable)]
    pub struct ReviewTag<'a> {
        pub id: i32,
        pub review_id: i32,
        pub name: &'a str,
    }

    #[derive(Insertable)]
    #[diesel(table_name = review_tags)]
    pub struct NewReviewTag {
        pub review_id: i32,
        pub name: String,
    }
}
