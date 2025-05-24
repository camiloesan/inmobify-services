use diesel::PgConnection;
use uuid::Uuid;

use crate::dto::new_user::NewUser;
use crate::dto::user::User;
use crate::dto::update_user::UpdateUser;

pub trait UsersRepository {
    fn create_user(user: NewUser, conn: &mut PgConnection) -> Option<String>;
    fn fetch_user_by_uuid(uuid: Uuid, conn: &mut PgConnection) -> Option<User>;
    fn update_user_by_uuid(
        uuid: Uuid,
        updated_user: UpdateUser,
        conn: &mut PgConnection,
    ) -> Option<User>;
    fn delete_user_by_uuid(uuid: Uuid, conn: &mut PgConnection) -> bool;
}
