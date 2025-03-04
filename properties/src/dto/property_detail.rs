use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Property {
    #[schema(example = "699f69e5-a2a4-4168-a535-b900a1c822be", required = true)]
    pub id: String,
    #[schema(example = "Claudia", required = true)]
    pub title: String,
    #[schema(example = "Sheinbaum", required = true)]
    pub description: String,
    pub n_rooms: i32,
    pub n_bathrooms: i32,
    pub sqm: i32,
    pub price: f32,
    pub owner_id: String,
    pub location_id: i32,
}
