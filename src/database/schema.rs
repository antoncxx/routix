// @generated automatically by Diesel CLI.

diesel::table! {
    certificates (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[sql_name = "type"]
        #[max_length = 20]
        type_ -> Varchar,
        certificate -> Text,
        private_key -> Text,
        #[max_length = 50]
        dns_provider -> Nullable<Varchar>,
        expires_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    proxy_hosts (id) {
        id -> Int4,
        #[max_length = 255]
        domain -> Varchar,
        #[max_length = 5]
        forward_schema -> Varchar,
        #[max_length = 255]
        forward_host -> Varchar,
        forward_port -> Int4,
        #[max_length = 255]
        certificate_name -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(certificates, proxy_hosts, users,);
