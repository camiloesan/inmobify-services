use crate::dal::{repository::PropertiesRepository, sch_models::Property};
use diesel::prelude::*;
use dotenvy::dotenv;
use log::error;
use std::env;

#[derive(Clone)]
pub struct PgProperties {
    url: String,
}

impl PgProperties {
    pub fn new(url: String) -> Self {
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
                error!("{}", e);
                panic!("Failed to establish connection");
            }
        }
    }
}

impl PropertiesRepository for PgProperties {
    fn fetch_top_properties(&self) -> Vec<Property> {
        use crate::dal::schema::properties::dsl::*;

        let conn = &mut self.get_connection();
        let result = properties
            .limit(10)
            .load::<Property>(conn);

        match result {
            Ok(list_properties) => list_properties,
            Err(e) => {
                error!("{}", e);
                vec![]
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_top_properties() {
        let repo = PgProperties::_test();
        let result = repo.fetch_top_properties();
        assert!(!result.is_empty());
    }
}