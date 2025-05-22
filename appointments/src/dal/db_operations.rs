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
    
    fn get_prospects_by_user_id(
        conn: &mut PgConnection,
        user_id: uuid::Uuid,
    ) -> Result<Vec<super::sch_models::ProspectSummary>, diesel::result::Error> {
        use crate::dal::schema::prospects::dsl::*;
        use crate::dal::schema::prospects::id as p_id;

        let result = prospects
            .select((
                p_id,
                name,
                last_name,
                email,
                phone,
                property_id,
            ))
            .filter(owner_id.eq(user_id))
            .order(created_at.desc())
            .load::<super::sch_models::ProspectSummary>(conn)?;
        Ok(result)
    }
}