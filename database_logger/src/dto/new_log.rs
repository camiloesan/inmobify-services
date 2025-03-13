use diesel::sql_types::Inet;
use serde::{Serialize, Deserialize };
use uuid::Uuid;
use ip_network::IpNetwork;

#[derive ( Debug, Clone, Serialize, Deserialize )]
pub struct NewLog {
    pub service: String,
    pub operation: i32,
    pub affected_table: String,
    pub element_id: String,
    pub ip: String,
    pub user: String,
}