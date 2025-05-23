use crate::dal::schema::prospects;

use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable, Debug, Queryable)]
#[diesel(table_name = prospects)]

pub struct NewProspect<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub property_id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
}

#[derive(Queryable, Debug)]
pub struct ProspectSummary {
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub property_id: Uuid,
}

