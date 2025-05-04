use crate::dal::{
    repository::PropertiesRepository,
    sch_models::{PropertyPreview, PropertyWithDetails},
    schema::{
        disposition_types,
        locations::{self},
        properties::{self, created_at},
        property_types, states,
    },
};
use diesel::prelude::*;
use dotenvy::dotenv;
use log::error;
use std::env;

use super::sch_models::{Location, NewLocation, NewProperty, State};

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
            .inner_join(locations::table.inner_join(states::table))
            .inner_join(property_types::table)
            .inner_join(disposition_types::table)
            .select((
                properties::all_columns(),
                locations::street,
                locations::house_number,
                locations::neighborhood,
                locations::zip_code,
                locations::city_name,
                states::name,
                locations::latitude,
                locations::longitude,
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
            .inner_join(locations::table.inner_join(states::table))
            .inner_join(property_types::table)
            .inner_join(disposition_types::table)
            .select((
                properties::all_columns,
                locations::street,
                locations::house_number,
                locations::neighborhood,
                locations::zip_code,
                locations::city_name,
                states::name,
                locations::latitude,
                locations::longitude,
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

    fn create_property(
        conn: &mut PgConnection,
        property: NewProperty,
    ) -> Result<uuid::Uuid, diesel::result::Error> {
        diesel::insert_into(properties::table)
            .values(&property)
            .execute(conn)?;

        Ok(property.id)
    }

    fn delete_property_by_uuid(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
    ) -> Result<i32, diesel::result::Error> {
        let result = diesel::delete(properties::table.filter(properties::id.eq(property_id)))
            .execute(conn)?;

        Ok(result as i32)
    }

    fn create_location(
        conn: &mut PgConnection,
        location: NewLocation,
    ) -> Result<i32, diesel::result::Error> {
        let location = diesel::insert_into(locations::table)
            .values(&location)
            .get_result::<Location>(conn)?;

        Ok(location.id)
    }

    fn delete_location_by_id(
        conn: &mut PgConnection,
        location_id: i32,
    ) -> Result<i32, diesel::result::Error> {
        let result =
            diesel::delete(locations::table.filter(locations::id.eq(location_id))).execute(conn)?;

        Ok(result as i32)
    }

    fn update_image_path(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
        image_path: String,
    ) -> Result<i32, diesel::result::Error> {
        let result = diesel::update(properties::table.filter(properties::id.eq(property_id)))
            .set(properties::img_path.eq(image_path))
            .execute(conn)?;

        Ok(result as i32)
    }

    fn get_states(conn: &mut PgConnection) -> Result<Vec<State>, diesel::result::Error> {
        use crate::dal::schema::states::dsl::*;

        let result = states.load(conn)?;

        Ok(result)
    }

    fn get_top_5_properties_by_user_id(
        conn: &mut PgConnection,
        user_id: uuid::Uuid,
    ) -> Result<Vec<super::sch_models::PropertyPreview>, diesel::result::Error> {
        use crate::dal::schema::properties::dsl::properties as properties_schema;

        let result = properties_schema
            .inner_join(locations::table.inner_join(states::table))
            .select((
                properties::id,
                properties::title,
                locations::street,
                locations::house_number,
                locations::neighborhood,
                locations::zip_code,
                locations::city_name,
                states::name,
            ))
            .filter(properties::owner_id.eq(user_id))
            .order(created_at.desc())
            .limit(5)
            .load::<PropertyPreview>(conn)?;

        Ok(result)
    }

    fn get_location_id_by_property_uuid(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
    ) -> Result<i32, diesel::result::Error> {
        let result = properties::table
            .filter(properties::id.eq(property_id))
            .select(properties::location_id)
            .first::<i32>(conn)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn test_get_top_5_properties_by_user_id() {
        let mut conn = PgProperties::_tests_get_connection();
        let user_id = Uuid::from_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
        let result = PgProperties::get_top_5_properties_by_user_id(&mut conn, user_id).unwrap();

        println!("{:?}", result);

        assert!(!result.is_empty());
    }

    #[test]
    fn get_location_id_by_property_uuid() {
        let mut conn = PgProperties::_tests_get_connection();
        let property_id = Uuid::from_str("d4a52262-48ee-4119-b9de-dfd80246a0d6").unwrap();
        let result =
            PgProperties::get_location_id_by_property_uuid(&mut conn, property_id).unwrap();
        println!("{}", result);

        assert!(result != 0)
    }

    #[test]
    fn test_fetch_states() {
        let mut conn = PgProperties::_tests_get_connection();
        let result = PgProperties::get_states(&mut conn).unwrap();

        assert!(!result.is_empty());
    }

    #[test]
    fn test_fetch_top_properties() {
        let mut conn = PgProperties::_tests_get_connection();
        let result = PgProperties::fetch_top_properties(&mut conn);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_update_property_image_path() {
        let mut conn = PgProperties::_tests_get_connection();

        let location = NewLocation {
            street: "test",
            house_number: "test",
            neighborhood: "test",
            zip_code: "test",
            latitude: "test",
            longitude: "test",
            city_name: "test",
            state_id: 1,
        };
        let result = PgProperties::create_location(&mut conn, location);
        let location_id = result.unwrap();
        println!("Location ID = {}", location_id);
        assert!(location_id > 0);

        let property = NewProperty {
            description: Some("test"),
            price: 1000.0,
            location_id,
            id: Uuid::new_v4(),
            title: "Test Property",
            img_path: "test.jpg",
            n_rooms: 3,
            n_bathrooms: 2,
            sqm: 100.0,
            priority: 1,
            owner_id: Uuid::new_v4(),
            property_type_id: 1,
            disposition_type_id: 1,
        };
        let result = PgProperties::create_property(&mut conn, property);
        let property_id = result.unwrap();
        println!("Property ID = {}", property_id);

        // main assert
        let image_path = "test.jpg";
        let result =
            PgProperties::update_image_path(&mut conn, property_id, image_path.to_string());
        assert!(result.is_ok());

        let result = PgProperties::delete_property_by_uuid(&mut conn, property_id);
        assert!(result.is_ok());

        let result = PgProperties::delete_location_by_id(&mut conn, location_id);
        assert!(result.is_ok());
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

    #[test]
    fn test_create_location() {
        let mut conn = PgProperties::_tests_get_connection();
        let location = NewLocation {
            street: "test",
            house_number: "test",
            neighborhood: "test",
            zip_code: "test",
            latitude: "test",
            longitude: "test",
            city_name: "test",
            state_id: 1,
        };
        let result = PgProperties::create_location(&mut conn, location);
        let location_id = result.unwrap();
        println!("Location ID = {}", location_id);
        assert!(location_id > 0);

        let result = PgProperties::delete_location_by_id(&mut conn, location_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_property() {
        let mut conn = PgProperties::_tests_get_connection();
        let location = NewLocation {
            street: "test",
            house_number: "test",
            neighborhood: "test",
            zip_code: "test",
            latitude: "test",
            longitude: "test",
            city_name: "test",
            state_id: 1,
        };
        let result = PgProperties::create_location(&mut conn, location);
        let location_id = result.unwrap();
        println!("Location ID = {}", location_id);
        assert!(location_id > 0);

        let property = NewProperty {
            description: Some("test"),
            price: 1000.0,
            location_id,
            id: Uuid::new_v4(),
            title: "Test Property",
            img_path: "test.jpg",
            n_rooms: 3,
            n_bathrooms: 2,
            sqm: 100.0,
            priority: 1,
            owner_id: Uuid::new_v4(),
            property_type_id: 1,
            disposition_type_id: 1,
        };
        let result = PgProperties::create_property(&mut conn, property);
        let property_id = result.unwrap();
        println!("Property ID = {}", property_id);

        let result = PgProperties::delete_property_by_uuid(&mut conn, property_id);
        assert!(result.is_ok());

        let result = PgProperties::delete_location_by_id(&mut conn, location_id);
        assert!(result.is_ok());
    }
}
