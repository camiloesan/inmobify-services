use diesel::PgConnection;

use super::sch_models::{NewLocation, NewProperty, PropertyPreview, PropertyWithDetails, State};

pub trait PropertiesRepository {
    fn fetch_top_properties(conn: &mut PgConnection) -> Vec<PropertyWithDetails>;
    fn fetch_property_details(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
    ) -> Option<PropertyWithDetails>;
    fn create_property(
        conn: &mut PgConnection,
        property: NewProperty,
    ) -> Result<uuid::Uuid, diesel::result::Error>;
    fn _delete_property(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
    ) -> Result<i32, diesel::result::Error>;
    fn create_location(
        conn: &mut PgConnection,
        location: NewLocation,
    ) -> Result<i32, diesel::result::Error>;
    fn _delete_location(
        conn: &mut PgConnection,
        location_id: i32,
    ) -> Result<i32, diesel::result::Error>;
    fn update_image_path(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
        image_path: String,
    ) -> Result<i32, diesel::result::Error>;
    fn get_states(conn: &mut PgConnection) -> Result<Vec<State>, diesel::result::Error>;
    fn get_top_5_properties_by_user_id(
        conn: &mut PgConnection,
        user_id: uuid::Uuid,
    ) -> Result<Vec<PropertyPreview>, diesel::result::Error>;
}
