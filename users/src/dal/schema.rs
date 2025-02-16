// @generated automatically by Diesel CLI.

diesel::table! {
    user_types (id) {
        id -> Int4,
        #[sql_name = "type"]
        #[max_length = 16]
        type_ -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 128]
        last_name -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        #[max_length = 10]
        phone -> Varchar,
        #[max_length = 36]
        password -> Varchar,
        created_at -> Timestamp,
        user_type_id -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    user_types,
    users,
);
