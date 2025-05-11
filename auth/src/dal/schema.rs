// @generated automatically by Diesel CLI.

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
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    users,
);
