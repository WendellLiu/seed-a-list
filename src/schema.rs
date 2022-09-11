// @generated automatically by Diesel CLI.

diesel::table! {
    review_tags (id) {
        id -> Integer,
        review_id -> Integer,
        name -> Varchar,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    reviews (id) {
        id -> Integer,
        external_author_id -> Varchar,
        external_id -> Varchar,
        source -> Varchar,
        content -> Nullable<Text>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    review_tags,
    reviews,
);
