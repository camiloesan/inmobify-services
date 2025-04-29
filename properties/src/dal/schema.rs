// @generated automatically by Diesel CLI.

diesel::table! {
    cities (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    disposition_types (id) {
        id -> Int4,
        #[max_length = 16]
        disposition -> Varchar,
    }
}

diesel::table! {
    locations (id) {
        id -> Int4,
        #[max_length = 255]
        street -> Varchar,
        #[max_length = 255]
        house_number -> Varchar,
        #[max_length = 255]
        neighborhood -> Varchar,
        #[max_length = 5]
        zip_code -> Varchar,
        #[max_length = 255]
        latitude -> Varchar,
        #[max_length = 255]
        longitude -> Varchar,
        #[max_length = 255]
        city_name -> Varchar,
        state_id -> Int4,
    }
}

diesel::table! {
    properties (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        img_path -> Varchar,
        #[max_length = 1024]
        description -> Nullable<Varchar>,
        n_rooms -> Int4,
        n_bathrooms -> Int4,
        sqm -> Float4,
        priority -> Int4,
        price -> Float4,
        owner_id -> Uuid,
        created_at -> Timestamp,
        location_id -> Int4,
        property_type_id -> Int4,
        disposition_type_id -> Int4,
    }
}

diesel::table! {
    property_status_history (id) {
        id -> Int4,
        property_id -> Uuid,
        status_id -> Int4,
        changed_at -> Timestamp,
    }
}

diesel::table! {
    property_statuses (id) {
        id -> Int4,
        #[max_length = 32]
        status_name -> Varchar,
    }
}

diesel::table! {
    property_types (id) {
        id -> Int4,
        #[sql_name = "type"]
        #[max_length = 16]
        type_ -> Varchar,
    }
}

diesel::table! {
    states (id) {
        id -> Int4,
        #[max_length = 24]
        name -> Varchar,
    }
}

diesel::joinable!(locations -> states (state_id));
diesel::joinable!(properties -> disposition_types (disposition_type_id));
diesel::joinable!(properties -> locations (location_id));
diesel::joinable!(properties -> property_types (property_type_id));
diesel::joinable!(property_status_history -> properties (property_id));
diesel::joinable!(property_status_history -> property_statuses (status_id));

diesel::allow_tables_to_appear_in_same_query!(
    cities,
    disposition_types,
    locations,
    properties,
    property_status_history,
    property_statuses,
    property_types,
    states,
);
