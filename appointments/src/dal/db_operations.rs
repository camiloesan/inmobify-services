use crate::dal::repository::AppointmentsRepository;
use diesel::prelude::*;

use super::sch_models::NewProspect;

#[derive(Clone)]
pub struct PgAppointments {}

impl PgAppointments {}

impl AppointmentsRepository for PgAppointments {
    fn create_prospect(
            conn: &mut PgConnection,
            prospect: NewProspect,
        ) -> Result<uuid::Uuid, diesel::result::Error> {
        use crate::dal::schema::prospects::dsl::*;

        diesel::insert_into(prospects)
        .values(&prospect)
        .execute(conn)?;
    Ok(prospect.id)
    }
}