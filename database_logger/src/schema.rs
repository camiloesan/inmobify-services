// @generated automatically by Diesel CLI.

diesel::table! {
    operation_logs (id) {
        id -> Int4,
        #[max_length = 36]
        service -> Varchar,
        operation -> Int4,
        #[max_length = 36]
        table -> Varchar,
        element_id -> Uuid,
        ip -> Inet,
        #[max_length = 128]
        user -> Varchar,
        date -> Timestamp,
    }
}

diesel::table! {
    operations (id) {
        id -> Int4,
        #[max_length = 36]
        name -> Varchar,
    }
}

diesel::joinable!(operation_logs -> operations (operation));

diesel::allow_tables_to_appear_in_same_query!(
    operation_logs,
    operations,
);
