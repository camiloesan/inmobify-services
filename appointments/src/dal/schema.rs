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

diesel::table! {
    transaction_types (id) {
        id -> Int4,
        #[max_length = 16]
        disposition -> Varchar,
    }
}

diesel::table! {
    transactions (id) {
        id -> Uuid,
        prospect_id -> Uuid,
        transaction_type_id -> Int4,
        date -> Timestamp,
        property_id -> Uuid,
    }
}

diesel::joinable!(transactions -> prospects (prospect_id));
diesel::joinable!(transactions -> transaction_types (transaction_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    prospects,
    transaction_types,
    transactions,
);
