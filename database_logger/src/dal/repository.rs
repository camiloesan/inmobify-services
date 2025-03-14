use crate::dto::new_log::NewLog;

pub trait DatabaseLoggerRepository {
    fn create_log(&self, log: NewLog) -> Option<String>;
    fn _delete_log_by_element_id(&self, element_id: uuid::Uuid) -> Option<uuid::Uuid>;
}