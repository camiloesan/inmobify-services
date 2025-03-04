use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PropertySummary {
    #[schema(example = "699f69e5-a2a4-4168-a535-b900a1c822be", required = true)]
    pub id: String,
    #[schema(example = "/assets/img1.jpg", required = true)]
    pub image_path: String,
    #[schema(example = "Casa renovada pet friendly", required = true)]
    pub title: String,
    #[schema(example = "Santa Catalina 12, Maestros Veracruzanos, 91000, Xalapa, Veracruz", required = true)]
    pub address: String,
    #[schema(example = "2,500,000.99", required = true)]
    pub price: f32,
    #[schema(example = "2", required = true)]
    pub n_rooms: i32,
    #[schema(example = "1", required = true)]
    pub n_bathrooms: i32,
    #[schema(example = "125", required = true)]
    pub sqm: i32,
    #[schema(example = "Sale/Rent", required = true)]
    pub disposition: String,
}
