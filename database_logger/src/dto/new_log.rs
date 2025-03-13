use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLog {
    pub service: String,
    pub operation: i32,
    pub affected_table: String,
    pub element_id: uuid::Uuid,
    pub ip: ipnetwork::IpNetwork,
    pub user: String,
}
