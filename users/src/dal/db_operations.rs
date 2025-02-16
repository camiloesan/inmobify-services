use crate::dal::repository::UsersRepository;
use crate::dal::sch_models::{NewUser, User};
use diesel::prelude::*;
use dotenvy::dotenv;
use log::error;
use std::env;
use uuid::Uuid;

use super::sch_models::UpdateUser;

#[derive(Clone)]
pub struct PgUsers {
    url: String,
}

impl PgUsers {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn _test() -> Self {
        dotenv().ok();
        let local_db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Self { url: local_db_url }
    }

    fn get_connection(&self) -> PgConnection {
        let conn = PgConnection::establish(&self.url);
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
    fn create_user(&self, user: crate::dto_models::NewUser) -> Option<String> {
        use crate::dal::schema::users;

        let uuid = Uuid::new_v4();
        let new_user = NewUser {
            id: uuid,
            name: &user.name,
            last_name: &user.last_name,
            email: &user.email,
            phone: &user.phone,
            password: &user.password,
            user_type_id: user.user_type_id,
        };

        let conn = &mut self.get_connection();
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

    fn get_user_by_uuid(&self, uuid: Uuid) -> Option<crate::dto_models::User> {
        use crate::dal::schema::users::dsl::*;

        let conn = &mut self.get_connection();
        let result = users
        .filter(id.eq(uuid))
        .select(User::as_select())
        .first::<User>(conn)
        .optional();

        match result {
            Ok(Some(user)) => Some(crate::dto_models::User {
                id: user.id.to_string(),
                name: user.name,
                last_name: user.last_name,
                email: user.email,
                phone: user.phone,
                created_at: user.created_at.to_string(),
                user_type_id: user.user_type_id,
            }),
            Ok(None) => None,
            Err(e) => {
                error!("{}", e);
                None
            }
        }
    }

    fn update_user_by_uuid(&self, uuid: Uuid, updated_user: UpdateUser) -> Option<crate::dto_models::User> {
        use crate::dal::schema::users::dsl::*;

        let conn = &mut self.get_connection();

        let result = diesel::update(users.filter(id.eq(uuid)))
        .set(updated_user)
        .get_result::<User>(conn);

        match result {
            Ok(user) => Some(crate::dto_models::User {
                id: user.id.to_string(),
                name: user.name,
                last_name: user.last_name,
                email: user.email,
                phone: user.phone,
                created_at: user.created_at.to_string(),
                user_type_id: user.user_type_id,
            }),
            Err(e) => {
                error!("{}", e);
                None
            }
        }
    }

    fn delete_user_by_uuid(&self, uuid: Uuid) -> bool {
        use crate::dal::schema::users::dsl::*;

        let conn = &mut self.get_connection();
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
        let repo = PgUsers::_test();
        let user = crate::dto_models::NewUser {
            name: "Mariana".to_string(),
            last_name: "Gonzáles Pérez".to_string(),
            email: "marg@gmail.com".to_string(),
            phone: "2288909021".to_string(),
            password: "699f69e5-a2a4-4168-a535-b900a1c822be".to_string(),
            user_type_id: 1,
        };

        // assertion
        let result = repo.create_user(user.clone());
        assert!(result.is_some());

        // tear down
        let uuid = Uuid::parse_str(result.unwrap().as_str()).unwrap();
        let delete_result = repo.delete_user_by_uuid(uuid);
        assert!(delete_result);
    }

    #[test]
    fn test_get_user() {
        // set up
        let repo = PgUsers::_test();
        let user = crate::dto_models::NewUser {
            name: "Mariana".to_string(),
            last_name: "Gonzáles Pérez".to_string(),
            email: "marg@gmail.com".to_string(),
            phone: "2288909021".to_string(),
            password: "699f69e5-a2a4-4168-a535-b900a1c822be".to_string(),
            user_type_id: 1,
        };

        // assertion 1
        let result = repo.create_user(user.clone());
        assert!(result.is_some());

        // main assertion
        let uuid = Uuid::parse_str(result.unwrap().as_str()).unwrap();
        let result = repo.get_user_by_uuid(uuid.clone());
        println!("{:?}", result);
        assert!(result.is_some());

        // tear down
        let delete_result = repo.delete_user_by_uuid(uuid);
        assert!(delete_result);
    }

    #[test]
    fn test_update_user() {
        // set up
        let repo = PgUsers::_test();
        let user = crate::dto_models::NewUser {
            name: "Mariana".to_string(),
            last_name: "Gonzáles Pérez".to_string(),
            email: "marg@gmail.com".to_string(),
            phone: "2288909021".to_string(),
            password: "699f69e5-a2a4-4168-a535-b900a1c822be".to_string(),
            user_type_id: 1,
        };

        // assertion 1
        let result = repo.create_user(user.clone());
        assert!(result.is_some());

        let updated_user = UpdateUser {
            name: Some("Emiliano".to_string()),
            last_name: None,
            email: None,
            phone: None,
            password: None,
            created_at: None,
            user_type_id: None,
        };

        // main assertion
        let uuid = Uuid::parse_str(result.unwrap().as_str()).unwrap();
        let result = repo.update_user_by_uuid(uuid.clone(), updated_user);
        println!("{:?}", result);
        assert!(result.is_some());

        // tear down
        let delete_result = repo.delete_user_by_uuid(uuid);
        assert!(delete_result);
    }

}
