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

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub password: &'a str,
}