use crate::dal::repository::AppointmentsRepository;
use diesel::prelude::*;

use super::sch_models::{NewProspect, NewTransaction};

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

    fn check_prospect_exists(
        conn: &mut PgConnection,
        check_property_id: uuid::Uuid,
        check_email: &str,
    ) -> Result<bool, diesel::result::Error> {
        use crate::dal::schema::prospects::id as p_id;
        use crate::dal::schema::prospects::dsl::*;
        use crate::dal::schema::prospects::dsl::{property_id, email};

        let exists = prospects
            .select(p_id)
            .filter(property_id.eq(check_property_id).and(email.eq(check_email)))
            .first::<uuid::Uuid>(conn)
            .optional()?
            .is_some();

        Ok(exists)
    }

    fn create_transaction(
        conn: &mut PgConnection,
        transaction: NewTransaction,
    ) -> Result<uuid::Uuid, diesel::result::Error> {
        use crate::dal::schema::transactions::dsl::*;

        diesel::insert_into(transactions)
        .values(&transaction)
        .execute(conn)?;

        Ok(transaction.id)
    }
}
