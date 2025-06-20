use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct NewTransaction {
    pub prospect_id: String,
    pub transaction_type_id: i32,
    pub property_id: String,
}
