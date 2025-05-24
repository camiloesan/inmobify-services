use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]

pub struct NewProspect {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub property_id: String,
    pub owner_id: String,
}