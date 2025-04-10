use diesel::PgConnection;

use super::sch_models::PropertyWithDetails;

pub trait PropertiesRepository {
    fn fetch_top_properties(conn: &mut PgConnection) -> Vec<PropertyWithDetails>;
    fn fetch_property_details(
        conn: &mut PgConnection,
        property_id: uuid::Uuid,
    ) -> Option<PropertyWithDetails>;
}
