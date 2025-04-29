use crate::dal::schema::{locations, properties};

use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = properties)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Property {
    pub id: Uuid,
    pub title: String,
    pub img_path: String,
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

#[derive(Insertable, Debug)]
#[diesel(table_name = properties)]
pub struct NewProperty<'a> {
    pub id: uuid::Uuid,
    pub title: &'a str,
    pub img_path: &'a str,
    pub description: Option<&'a str>,
    pub n_rooms: i32,
    pub n_bathrooms: i32,
    pub sqm: f32,
    pub priority: i32,
    pub price: f32,
    pub owner_id: uuid::Uuid,
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
    pub latitude: String,
    pub longitude: String,
    pub city_name: String,
    pub state_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = locations)]
pub struct NewLocation<'a> {
    pub street: &'a str,
    pub house_number: &'a str,
    pub neighborhood: &'a str,
    pub zip_code: &'a str,
    pub latitude: &'a str,
    pub longitude: &'a str,
    pub city_name: &'a str,
    pub state_id: i32,
}

#[derive(Queryable, Debug)]
pub struct State {
    pub id: i32,
    pub name: String,
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
