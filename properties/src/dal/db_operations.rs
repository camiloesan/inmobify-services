use crate::{
    dal::{repository::PropertiesRepository, schema::properties::disposition_type_id},
    load_env,
};
use diesel::prelude::*;
use log::error;
use std::env;

use super::sch_models::{
    Location, NewLocation, NewProperty, PropertyPreview, PropertyWithDetails, State,
    UpdateLocation, UpdateProperty,
};

#[derive(Clone)]
pub struct PgProperties {}

impl PgProperties {
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

impl PropertiesRepository for PgProperties {
    fn fetch_top_properties(conn: &mut PgConnection) -> Vec<PropertyWithDetails> {
        use crate::dal::schema::disposition_types::dsl::*;
        use crate::dal::schema::locations::dsl::*;
        use crate::dal::schema::properties::dsl::properties;
        use crate::dal::schema::property_types::dsl as property_types;
        use crate::dal::schema::states::dsl as states;

        let result = properties
            .limit(10)
            .inner_join(locations.inner_join(states::states))
            .inner_join(property_types::property_types)
            .inner_join(disposition_types)
            .select((
                properties::all_columns(),
                street,
                house_number,
                neighborhood,
                zip_code,
                city_name,
                states::name,
                states::id,
                latitude,
                longitude,
                property_types::type_,
                property_types::id,
                disposition,
                disposition_type_id,
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
        use crate::dal::schema::disposition_types::dsl::*;
        use crate::dal::schema::locations::dsl::*;
        use crate::dal::schema::properties::dsl::*;
        use crate::dal::schema::properties::id as p_id;
        use crate::dal::schema::property_types::dsl as property_types;
        use crate::dal::schema::states::dsl as states;

        let result = properties
            .inner_join(locations.inner_join(states::states))
            .inner_join(property_types::property_types)
            .inner_join(disposition_types)
            .select((
                properties::all_columns(),
                street,
                house_number,
                neighborhood,
                zip_code,
                city_name,
                states::name,
                states::id,
                latitude,
                longitude,
                property_types::type_,
                property_types::id,
                disposition,
                disposition_type_id,
            ))
            .filter(p_id.eq(property_id))
            .first::<PropertyWithDetails>(conn);

        match result {
            Ok(property) => {
                println!("{:?}", property);
                Some(property)
            }
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
        use crate::dal::schema::properties::dsl::*;

        diesel::insert_into(properties)
            .values(&property)
            .execute(conn)?;

        Ok(property.id)
    }

    fn update_property_location_transaction(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
        updated_property: UpdateProperty,
        updated_location: UpdateLocation,
    ) -> Result<(), diesel::result::Error> {
        use crate::dal::schema::locations::id as l_id;
        use crate::dal::schema::locations::table as locations;
        use crate::dal::schema::properties::dsl::*;
        use crate::dal::schema::properties::id as p_id;

        let result = conn.transaction(|conn| {
            let loc_id = properties
                .select(location_id)
                .filter(p_id.eq(property_id))
                .get_result::<i32>(conn)?;

            diesel::update(properties.filter(p_id.eq(property_id)))
                .set(&updated_property)
                .execute(conn)?;

            diesel::update(locations.filter(l_id.eq(loc_id)))
                .set(&updated_location)
                .execute(conn)?;

            diesel::result::QueryResult::Ok(())
        });

        result
    }

    fn delete_property_by_uuid(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
    ) -> Result<i32, diesel::result::Error> {
        use crate::dal::schema::properties::dsl::*;

        let result = diesel::delete(properties.filter(id.eq(property_id))).execute(conn)?;

        Ok(result as i32)
    }

    fn create_location(
        conn: &mut PgConnection,
        location: NewLocation,
    ) -> Result<i32, diesel::result::Error> {
        use crate::dal::schema::locations::dsl::*;

        let location = diesel::insert_into(locations)
            .values(&location)
            .get_result::<Location>(conn)?;

        Ok(location.id)
    }

    fn delete_location_by_id(
        conn: &mut PgConnection,
        location_id: i32,
    ) -> Result<i32, diesel::result::Error> {
        use crate::dal::schema::locations::dsl::*;

        let result = diesel::delete(locations.filter(id.eq(location_id))).execute(conn)?;

        Ok(result as i32)
    }

    fn update_image_path(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
        image_path: String,
    ) -> Result<i32, diesel::result::Error> {
        use crate::dal::schema::properties::dsl::*;

        let result = diesel::update(properties.filter(id.eq(property_id)))
            .set(img_path.eq(image_path))
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
        use crate::dal::schema::locations::dsl::*;
        use crate::dal::schema::properties::dsl::*;
        use crate::dal::schema::properties::id as p_id;
        use crate::dal::schema::states::dsl as states;

        let result = properties
            .inner_join(locations.inner_join(states::states))
            .select((
                p_id,
                title,
                street,
                house_number,
                neighborhood,
                zip_code,
                city_name,
                states::name,
            ))
            .filter(owner_id.eq(user_id))
            .order(created_at.desc())
            .limit(5)
            .load::<PropertyPreview>(conn)?;

        Ok(result)
    }

    fn get_location_id_by_property_uuid(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
    ) -> Result<i32, diesel::result::Error> {
        use crate::dal::schema::properties::dsl::*;

        let result = properties
            .filter(id.eq(property_id))
            .select(location_id)
            .first::<i32>(conn)?;

        Ok(result)
    }

    fn update_property_priority(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
        new_priority: i32,
    ) -> Result<i32, diesel::result::Error> {
        use crate::dal::schema::properties::dsl::*;

        let result = diesel::update(properties.filter(id.eq(property_id)))
            .set((
                priority.eq(new_priority),
                modified_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(conn)?;

        Ok(result as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn test_update_property_priority() {
        let mut conn = PgProperties::_tests_get_connection();

        // set up
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
            priority: 0,
            owner_id: Uuid::new_v4(),
            property_type_id: 1,
            disposition_type_id: 1,
        };
        let result = PgProperties::create_property(&mut conn, property);
        let property_id = result.unwrap();

        // main assert
        let result = PgProperties::update_property_priority(&mut conn, property_id, 1);
        assert!(result.is_ok() && result.unwrap() == 1);

        // tear down
        let result = PgProperties::delete_property_by_uuid(&mut conn, property_id);
        assert!(result.is_ok());
        let result = PgProperties::delete_location_by_id(&mut conn, location_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_property_location_transaction() {
        let mut conn = PgProperties::_tests_get_connection();

        // set up
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

        let updated_location = UpdateLocation {
            street: Some("test".to_string()),
            house_number: Some("test".to_string()),
            neighborhood: Some("test".to_string()),
            zip_code: Some("test".to_string()),
            latitude: Some("test".to_string()),
            longitude: Some("test".to_string()),
            city_name: Some("test".to_string()),
            state_id: Some(1),
        };

        let updated_property = UpdateProperty {
            title: Some("new".to_string()),
            img_path: Some("new".to_string()),
            description: Some("new".to_string()),
            n_rooms: Some(1),
            n_bathrooms: Some(1),
            sqm: Some(1.1),
            priority: Some(1),
            price: Some(1.1),
            modified_at: Some(chrono::Utc::now().naive_utc()),
            property_type_id: Some(1),
            disposition_type_id: Some(1),
        };

        // main assert
        let result = PgProperties::update_property_location_transaction(
            &mut conn,
            property_id,
            updated_property,
            updated_location,
        );
        assert!(result.is_ok());

        // tear down
        let result = PgProperties::delete_property_by_uuid(&mut conn, property_id);
        assert!(result.is_ok());
        let result = PgProperties::delete_location_by_id(&mut conn, location_id);
        assert!(result.is_ok());
    }

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

        // set up
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

        // main assert
        let result =
            PgProperties::get_location_id_by_property_uuid(&mut conn, property_id).unwrap();
        assert!(result != 0);

        // tear down
        let result = PgProperties::delete_property_by_uuid(&mut conn, property_id);
        assert!(result.is_ok());
        let result = PgProperties::delete_location_by_id(&mut conn, location_id);
        assert!(result.is_ok());
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
