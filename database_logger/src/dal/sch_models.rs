use crate::dal::schema::operation_logs;

use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = operation_logs)]
pub struct NewOperationLog<'a> {
    pub service: &'a str,
    pub operation: i32,
    pub affected_table: &'a str,
    pub element_id: uuid::Uuid,
    pub ip: ipnetwork::IpNetwork,
    pub user: &'a str,
}
