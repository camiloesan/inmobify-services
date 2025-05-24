use diesel::PgConnection;

use super::sch_models::{NewProspect, ProspectSummary};

pub trait AppointmentsRepository {
    fn create_prospect(
        conn: &mut PgConnection,
        prospect: NewProspect,
    ) -> Result<uuid::Uuid, diesel::result::Error>;
    fn get_prospects_by_user_id(
        conn: &mut PgConnection,
        user_id: uuid::Uuid,
    ) -> Result<Vec<ProspectSummary>, diesel::result::Error>;
    fn check_prospect_exists(
        conn: &mut PgConnection,
        check_property_id: uuid::Uuid,
        check_email: &str, 
    ) -> Result<bool, diesel::result::Error>;
}