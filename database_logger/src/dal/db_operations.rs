use crate::dal::repository::DatabaseLoggerRepository;
use crate::dal::sch_models::NewOperationLog;
use diesel::prelude::*;
use dotenvy::dotenv;
use log::error;
use std::env;

#[derive(Clone)]
pub struct PgDatabaseLogs {
    url: String,
}

impl PgDatabaseLogs {
    pub fn _new(url: String) -> Self {
        Self { url }
    }

    pub fn _test() -> Self {
        dotenv().ok();
        let local_db_url = env::var("LOCAL_DB_URL").expect("LOCAL_DB_URL must be set");
        Self { url: local_db_url }
    }

    fn get_connection(&self) -> PgConnection {
        let conn = PgConnection::establish(&self.url);
        match conn {
            Ok(result) => result,
            Err(e) => {
                panic!("Failed to connect to database: {}", e);
            }
        }
    }
}

impl DatabaseLoggerRepository for PgDatabaseLogs {
    fn create_log(&self, log: crate::dto::new_log::NewLog) -> Option<String> {
        use crate::dal::schema::operation_logs;

        let new_log = NewOperationLog {
            service: &log.service,
            operation: log.operation,
            affected_table: &log.affected_table,
            element_id: log.element_id,
            ip: log.ip,
            user: &log.user,
        };

        let conn = &mut self.get_connection();
        let result = diesel::insert_into(operation_logs::table)
            .values(&new_log)
            .execute(conn);

        match result {
            Ok(_) => Some(new_log.service.to_string()),
            Err(e) => {
                error!("Failed to create new log: {}", e);
                None
            }
        }
    }

    fn _delete_log_by_element_id(&self, log_id: uuid::Uuid) -> Option<uuid::Uuid> {
        use crate::dal::schema::operation_logs::dsl::*;

        let conn = &mut self.get_connection();
        let result = conn.transaction(|conn| {
            diesel::delete(operation_logs.filter(element_id.eq(log_id))).execute(conn)
        });

        match result {
            Ok(_) => Some(log_id),
            Err(e) => {
                error!("Failed to delete log: {}", e);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dto::new_log;

    use super::*;

    #[test]
    fn test_fetch_top_properties() {
        // set up
        let repo = PgDatabaseLogs::_test();

        let new_log = new_log::NewLog {
            service: "service".to_string(),
            operation: 1,
            affected_table: "affected_table".to_string(),
            element_id: uuid::Uuid::new_v4(),
            ip: "10.1.9.32/16".parse().unwrap(),
            user: "user".to_string(),
        };

        let result = repo.create_log(new_log.clone());
        println!("{}", result.clone().unwrap());
        // main assert
        assert!(result.is_some());

        // tear down
        let result = repo._delete_log_by_element_id(new_log.element_id);
        println!("{}", result.clone().unwrap());
        assert!(result.is_some());
    }
}
