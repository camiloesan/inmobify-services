use uuid::Uuid;

use crate::dto_models::{NewUser, User};

use super::sch_models::UpdateUser;


pub trait UsersRepository {
    fn create_user(&self, user: NewUser) -> Option<String>;
    fn get_user_by_uuid(&self, uuid: Uuid) -> Option<User>;
    fn update_user_by_uuid(&self, uuid: Uuid, updated_user: UpdateUser) -> Option<User>;
    fn delete_user_by_uuid(&self, uuid: Uuid) -> bool;
}