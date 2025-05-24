use crate::dal::repository::UsersRepository;
use crate::dal::sch_models::{NewUser, User, UpdateUser};
use crate::{dto, load_env};
use diesel::prelude::*;
use log::error;
use std::env;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgUsers {}

impl PgUsers {
    fn _tests_get_connection() -> PgConnection {
        load_env();
        let local_db_url = env::var("LOCAL_DB_URL").expect("LOCAL_DB_URL must be set");
        let conn = PgConnection::establish(&local_db_url);
        match conn {
            Ok(result) => result,
            Err(e) => {
                error!("{}", e);
                panic!("Failed to establish connection");
            }
        }
    }
}

impl UsersRepository for PgUsers {
    fn create_user(user: dto::new_user::NewUser, conn: &mut PgConnection) -> Option<String> {
        use crate::dal::schema::users;

        let uuid = Uuid::new_v4();
        let new_user = NewUser {
            id: uuid,
            name: &user.name,
            last_name: &user.last_name,
            email: &user.email,
            phone: &user.phone,
            password: &user.password,
        };

        let result = diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn);

        match result {
            Ok(_) => Some(uuid.to_string()),
            Err(e) => {
                error!("{}", e);
                None
            }
        }
    }

    fn fetch_user_by_uuid(uuid: Uuid, conn: &mut PgConnection) -> Option<crate::dto::user::User> {
        use crate::dal::schema::users::dsl::*;

        let result = users
            .filter(id.eq(uuid))
            .select(User::as_select())
            .first::<User>(conn)
            .optional();

        match result {
            Ok(Some(user)) => Some(crate::dto::user::User {
                id: user.id.to_string(),
                name: user.name,
                last_name: user.last_name,
                email: user.email,
                phone: user.phone,
                created_at: user.created_at.to_string(),
            }),
            Ok(None) => None,
            Err(e) => {
                error!("Couldn't fetch user from database: {}", e);
                None
            }
        }
    }

    fn update_user_by_uuid(
        uuid: Uuid,
        updated_user: dto::update_user::UpdateUser,
        conn: &mut PgConnection,
    ) -> Option<crate::dto::user::User> {
        use crate::dal::schema::users::dsl::*;

        let diesel_updated_user = UpdateUser {
            name: updated_user.name,
            last_name: updated_user.last_name,
            email: updated_user.email,
            phone: updated_user.phone,
            password: updated_user.password,
        };

        let result = diesel::update(users.filter(id.eq(uuid)))
            .set(diesel_updated_user)
            .returning(User::as_select())
            .get_result::<User>(conn);

        match result {
            Ok(user) => Some(crate::dto::user::User {
                id: user.id.to_string(),
                name: user.name,
                last_name: user.last_name,
                email: user.email,
                phone: user.phone,
                created_at: user.created_at.to_string(),
            }),
            Err(e) => {
                error!("{}", e);
                None
            }
        }
    }

    fn delete_user_by_uuid(uuid: Uuid, conn: &mut PgConnection) -> bool {
        use crate::dal::schema::users::dsl::*;

        let result =
            conn.transaction(|conn| diesel::delete(users.filter(id.eq(uuid))).execute(conn));

        match result {
            Ok(_) => true,
            Err(e) => {
                error!("{}", e);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        // set up
        let user = crate::dto::new_user::NewUser {
            name: "Mariana".to_string(),
            last_name: "Gonzáles Pérez".to_string(),
            email: "marg@gmail.com".to_string(),
            phone: "2288909021".to_string(),
            password: "699f69e5-a2a4-4168-a535-b900a1c822be".to_string(),
        };

        // assertion
        let mut conn = PgUsers::_tests_get_connection();
        let result = PgUsers::create_user(user.clone(), &mut conn);
        assert!(result.is_some());

        // tear down
        let uuid = Uuid::parse_str(result.unwrap().as_str()).unwrap();
        let delete_result = PgUsers::delete_user_by_uuid(uuid, &mut conn);
        assert!(delete_result);
    }

    #[test]
    fn test_fetch_user() {
        // set up
        let user = crate::dto::new_user::NewUser {
            name: "Mariana".to_string(),
            last_name: "Gonzáles Pérez".to_string(),
            email: "marg@gmail.com".to_string(),
            phone: "2288909021".to_string(),
            password: "699f69e5-a2a4-4168-a535-b900a1c822be".to_string(),
        };

        // create assertion
        let mut conn = PgUsers::_tests_get_connection();
        let result = PgUsers::create_user(user.clone(), &mut conn);
        assert!(result.is_some());

        // main assertion
        let uuid = Uuid::parse_str(result.unwrap().as_str()).unwrap();
        let result = PgUsers::fetch_user_by_uuid(uuid.clone(), &mut conn);
        println!("{:?}", result);
        assert!(result.is_some());

        // tear down
        let delete_result = PgUsers::delete_user_by_uuid(uuid, &mut conn);
        assert!(delete_result);
    }

    #[test]
    fn test_update_user() {
        // github actions test
        // set up
        let user = crate::dto::new_user::NewUser {
            name: "Mariana".to_string(),
            last_name: "Gonzáles Pérez".to_string(),
            email: "marg@gmail.com".to_string(),
            phone: "2288909021".to_string(),
            password: "699f69e5-a2a4-4168-a535-b900a1c822be".to_string(),
        };

        // assertion 1
        let mut conn = PgUsers::_tests_get_connection();
        let result = PgUsers::create_user(user.clone(), &mut conn);
        assert!(result.is_some());

        let updated_user = crate::dto::update_user::UpdateUser {
            name: Some("Emiliano".to_string()),
            last_name: None,
            email: None,
            phone: None,
            password: None,
        };

        // main assertion
        let uuid = Uuid::parse_str(result.unwrap().as_str()).unwrap();
        let result = PgUsers::update_user_by_uuid(uuid.clone(), updated_user, &mut conn);
        println!("{:?}", result);
        assert!(result.is_some());

        // tear down
        let delete_result = PgUsers::delete_user_by_uuid(uuid, &mut conn);
        assert!(delete_result);
    }
}
