use crate::dal::schema::{operation_logs, operations};

use diesel::prelude::*;
use uuid::Uuid;
use diesel::Insertable;
use diesel::pg::sql_types::Inet;
use ip_network::IpNetwork;

#[derive(Insertable)]
#[diesel(table_name = operation_logs)]
pub struct NewOperationLog<'a> {
    pub service: &'a str,
    pub operation: i32,
    pub affected_table: &'a str,
    pub element_id: Uuid,
    pub ip: &'a str, // THIS ONE DOESN'T ALLOW Inet OR IpNetwork
    pub user: &'a str,
}