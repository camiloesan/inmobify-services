use crate::dal::schema::users;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}