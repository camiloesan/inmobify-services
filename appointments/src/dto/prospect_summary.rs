use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProspectSummary {
    pub id: String,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub property_id: String,
}