use crate::dal::schema::{properties, locations};

use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = properties)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Property {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub n_rooms: i32,
    pub n_bathrooms: i32,
    pub sqm: f32,
    pub priority: i32,
    pub price: f32,
    pub owner_id: Uuid,
    pub created_at: NaiveDateTime,
    pub location_id: i32,
    pub property_type_id: i32,
    pub disposition_type_id: i32,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Location {
    pub id: i32,
    pub street: String,
    pub house_number: String,
    pub neighborhood: String,
    pub zip_code: String,
    pub city_id: i32,
    pub state_id: i32,
}

#[derive(Queryable, Debug)]
pub struct PropertyWithDetails {
    pub property: Property,
    pub street: String,
    pub house_number: String,
    pub neighborhood: String,
    pub zip_code: String,
    pub city_name: String,
    pub state_name: String,
    pub property_type: String,
    pub disposition: String,
}