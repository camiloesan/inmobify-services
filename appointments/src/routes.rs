use std::str::FromStr;

use crate::dto::check_prospect::CheckProspect;
use crate::dto::new_prospect::NewProspect;
use crate::DbPool;
use crate::dal::{db_operations::PgAppointments, repository::AppointmentsRepository};
use actix_web::post;
use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};
use log::{error, info};
use uuid::Uuid;

/// Create a new prospect.
#[post("/prospect")]
pub async fn create_prospect(
    pool: web::Data<DbPool>,
    new_prospect: web::Json<NewProspect>,
) -> impl Responder {
    info!("request to create prospect received");

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        let prospect = crate::dal::sch_models::NewProspect {
            id: Uuid::new_v4(),
            name: &new_prospect.name,
            last_name: &new_prospect.last_name,
            email: &new_prospect.email,
            phone: &new_prospect.phone,
            property_id: Uuid::from_str(&new_prospect.property_id).unwrap(),
            owner_id: Uuid::from_str(&new_prospect.owner_id).unwrap(),
        };
        let prospect_uuid = PgAppointments::create_prospect(&mut conn, prospect).unwrap();

        prospect_uuid

    })
    .await;

    match result {
        Ok(prospect_uuid) => HttpResponse::Ok().json(prospect_uuid.to_string()),
        Err(err) => {
            error!("Failed to create prospect: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Get prospects
#[get("/user-prospects/{id}")]
pub async fn get_user_prospects(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
) -> HttpResponse {
    info!("Getting user prospects");

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        PgAppointments::get_prospects_by_user_id(&mut conn, Uuid::from_str(&user_id).unwrap())
            .unwrap()
    })
    .await;

    match result {
        Ok(prospects) => {
            let dto = prospects
            .into_iter()
            .map(|prospect| crate::dto::prospect_summary::ProspectSummary {
                id: prospect.id.to_string(),
                name: prospect.name,
                last_name: prospect.last_name,
                email: prospect.email,
                phone: prospect.phone,
                property_id: prospect.property_id.to_string(),
            })
            .collect::<Vec<_>>();
        HttpResponse::Ok().json(dto)
        }
        Err(e) => {
            error!("Error getting user prospects: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Check if a prospect exists for a given property_id and email
#[post("/prospect/exists")]
pub async fn check_prospect_exists(
    pool: web::Data<DbPool>,
    input: web::Json<CheckProspect>,
) -> impl Responder {
    info!("request to check if prospect exists received");

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        let id = Uuid::from_str(&input.property_id).unwrap();
        let exists = PgAppointments::check_prospect_exists(&mut conn, id, &input.email).unwrap();
        exists
    })
    .await;

    match result {
        Ok(exists) => HttpResponse::Ok().json(exists),
        Err(err) => {
            error!("Failed to check prospect existence: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
