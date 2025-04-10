use crate::dal::{
    repository::PropertiesRepository,
    sch_models::PropertyWithDetails,
    schema::{
        cities, disposition_types,
        locations::{self},
        properties, property_types, states,
    },
};
use diesel::prelude::*;
use dotenvy::dotenv;
use log::error;
use std::env;

#[derive(Clone)]
pub struct PgProperties {}

impl PgProperties {
    fn _tests_get_connection() -> PgConnection {
        dotenv().ok();
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

impl PropertiesRepository for PgProperties {
    fn fetch_top_properties(conn: &mut PgConnection) -> Vec<PropertyWithDetails> {
        use crate::dal::schema::properties::dsl::properties;

        let result = properties
            .limit(10)
            .inner_join(
                locations::table
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

    fn fetch_property_details(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
    ) -> Option<PropertyWithDetails> {
        let result = properties::table
            .inner_join(
                locations::table
                    .inner_join(cities::table)
                    .inner_join(states::table),
            )
            .inner_join(property_types::table)
            .inner_join(disposition_types::table)
            .select((
                properties::all_columns,
                locations::street,
                locations::house_number,
                locations::neighborhood,
                locations::zip_code,
                cities::name,
                states::name,
                property_types::type_,
                disposition_types::disposition,
            ))
            .filter(properties::id.eq(property_id))
            .first::<PropertyWithDetails>(conn);

        match result {
            Ok(property) => Some(property),
            Err(e) => {
                error!("Error fetching property details: {}", e);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_fetch_top_properties() {
        let mut conn = PgProperties::_tests_get_connection();
        let result = PgProperties::fetch_top_properties(&mut conn);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_fetch_property_details() {
        let mut conn = PgProperties::_tests_get_connection();
        let property_id = uuid::Uuid::new_v4();
        let result = PgProperties::fetch_property_details(&mut conn, property_id);
        assert!(result.is_none());
    }

    #[test]
    fn test_fetch_property_details_exists() {
        let mut conn = PgProperties::_tests_get_connection();
        let property_id = Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let result = PgProperties::fetch_property_details(&mut conn, property_id);
        assert!(result.is_some());
    }
}
