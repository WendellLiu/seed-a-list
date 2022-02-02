table! {
    reviews (id) {
        id -> Integer,
        external_author_id -> Varchar,
        external_id -> Varchar,
        source -> Varchar,
        content -> Nullable<Text>,
        tags -> Nullable<Json>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}
