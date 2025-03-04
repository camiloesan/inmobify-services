use crate::dal::schema::properties;

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