use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PropertyDetail {
    pub id: String,
    pub title: String,
    pub image_path: String,
    pub description: String,
    pub n_rooms: i32,
    pub n_bathrooms: i32,
    pub sqm: f32,
    pub price: f32,
    pub street: String,
    pub owner_id: String,
    pub created_at: String,
    pub house_number: String,
    pub neighborhood: String,
    pub zip_code: String,
    pub city: String,
    pub state: String,
    pub property_type: String,
    pub disposition: String,
    // pub location_id: i32,
}
