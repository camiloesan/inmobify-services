use crate::dal::schema::prospects;

use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable, Debug)]
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

