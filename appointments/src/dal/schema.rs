// @generated automatically by Diesel CLI.

diesel::table! {
    prospects (id) {
        id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 128]
        last_name -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        #[max_length = 10]
        phone -> Varchar,
        created_at -> Timestamp,
        property_id -> Uuid,
        owner_id -> Uuid,
    }
}
