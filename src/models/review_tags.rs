pub mod t {
    use crate::schema::review_tags;

    #[derive(Queryable)]
    pub struct ReviewTag<'a> {
        pub id: i32,
        pub review_id: i32,
        pub name: &'a str,
    }

    #[derive(Insertable)]
    #[table_name = "review_tags"]
    pub struct NewReviewTag<'a> {
        pub review_id: i32,
        pub name: &'a str,
    }
}
