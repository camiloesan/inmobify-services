use diesel::PgConnection;

use super::sch_models::PropertyWithDetails;

pub trait PropertiesRepository {
    fn fetch_top_properties(conn: &mut PgConnection) -> Vec<PropertyWithDetails>;
}