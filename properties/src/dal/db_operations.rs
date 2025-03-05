use crate::dal::{
    repository::PropertiesRepository,
    sch_models::PropertyWithDetails,
    schema::{
        cities, disposition_types, locations::{self}, property_types, states
    },
};
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
    fn fetch_top_properties(&self) -> Vec<PropertyWithDetails> {
        use crate::dal::schema::properties::dsl::properties;

        let conn = &mut self.get_connection();
        let result = properties
            .limit(10)
            .inner_join(locations::table
                .inner_join(cities::table)
                .inner_join(states::table),
            )
            .inner_join(property_types::table)
            .inner_join(disposition_types::table)
            .select((
                properties::all_columns(),
                locations::street,
                locations::house_number,
                locations::neighborhood,
                locations::zip_code,
                cities::name,
                states::name,
                property_types::type_,
                disposition_types::disposition,
            ))
            .load::<PropertyWithDetails>(conn);

        match result {
            Ok(list_properties) => list_properties,
            Err(e) => {
                error!("{}", e);
                vec![]
            }
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
        print!("{:?}", result);
        assert!(!result.is_empty());
    }
}
