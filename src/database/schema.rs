// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 20]
        role -> Varchar,
        scopes -> Array<Nullable<Text>>,
        created_at -> Timestamptz,
    }
}
