use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdatedProperty {
    pub title: String,
    pub img_path: String,
    pub description: String,
    pub n_rooms: i32,
    pub n_bathrooms: i32,
    pub sqm: f32,
    pub priority: i32,
    pub price: f32,
    pub property_type_id: i32,
    pub disposition_type_id: i32,
    pub street: String,
    pub house_number: String,
    pub neighborhood: String,
    pub zip_code: String,
    pub latitude: String,
    pub longitude: String,
    pub city_name: String,
    pub state_id: i32,
}
