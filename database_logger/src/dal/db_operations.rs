use crate::dal::repository::DatabaseLoggerRepository;
use crate::dal::sch_models::NewOperationLog;
use diesel::prelude::*;
use dotenvy::dotenv;
use log::error;
use std::env;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgDatabaseLogs {
    url: String,
}

impl PgDatabaseLogs {
    pub fn new(url: String) -> Self { Self { url } }

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
            element_id: Uuid::parse_str(&log.element_id).unwrap() ,
            ip: &log.ip,
            user: &log.user
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
            },
        }
    }
}