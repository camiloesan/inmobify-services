use crate::dal::schema::users;
use crate::dal::schema::user_types;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = user_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserType {
    pub id: i32,
    pub type_: String,
}

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
    pub user_type_id: i32,
}