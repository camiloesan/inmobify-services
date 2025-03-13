use diesel::PgConnection;
use crate::dto::new_log::NewLog;

pub trait DatabaseLoggerRepository {
    fn create_log(&self, log: NewLog) -> Option<String>;
}