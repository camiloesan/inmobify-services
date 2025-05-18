use diesel::PgConnection;

use super::sch_models::NewProspect;

pub trait AppointmentsRepository {
    fn create_prospect(
        conn: &mut PgConnection,
        prospect: NewProspect,
    ) -> Result<uuid::Uuid, diesel::result::Error>;
}