use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PropertySummary {
    #[schema(example = "699f69e5-a2a4-4168-a535-b900a1c822be", required = true)]
    pub id: String,
    #[schema(example = "/images/img1.jpg", required = true)]
    pub image_path: String,
    #[schema(example = "Casa en renta renovada pet friendly", required = true)]
    pub title: String,
    #[schema(example = "Santa Catalina", required = true)]
    pub street: String,
    #[schema(example = "12", required = true)]
    pub house_number: String,
    #[schema(example = "Valle de Oriente", required = true)]
    pub neighborhood: String,
    #[schema(example = "91000", required = true)]
    pub zip_code: String,
    #[schema(example = "Xalapa", required = true)]
    pub city: String,
    #[schema(example = "Veracruz", required = true)]
    pub state: String,
    #[schema(example = "2,500,000.99", required = true)]
    pub price: f32,
    #[schema(example = "2", required = true)]
    pub n_rooms: i32,
    #[schema(example = "1", required = true)]
    pub n_bathrooms: i32,
    #[schema(example = "125", required = true)]
    pub sqm: f32,
    pub property_type: String,
    #[schema(example = "Sale/Rent", required = true)]
    pub disposition: String,
}
