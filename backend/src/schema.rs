// @generated automatically by Diesel CLI.

diesel::table! {
    photo (photo_id) {
        photo_id -> Int4,
        post_id -> Int4,
        #[max_length = 128]
        description -> Nullable<Varchar>,
        #[max_length = 128]
        photographer -> Nullable<Varchar>,
        #[max_length = 128]
        photo_path -> Varchar,
        time_taken -> Timestamp,
    }
}

diesel::table! {
    post (post_id) {
        post_id -> Int4,
        #[max_length = 2048]
        description -> Nullable<Varchar>,
        like_count -> Nullable<Int4>,
        #[max_length = 128]
        song -> Nullable<Varchar>,
    }
}

diesel::joinable!(photo -> post (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    photo,
    post,
);
