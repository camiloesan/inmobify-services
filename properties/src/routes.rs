use std::str::FromStr;

use crate::dto::property_detail::PropertyDetail;
use crate::DbPool;
use crate::{
    dal::{db_operations::PgProperties, repository::PropertiesRepository},
    dto::property_summary::PropertySummary,
};
use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};
use log::{error, info};

/// Get a list of boosted properties.
#[utoipa::path(
    responses(
        (status = 200, description = "Retrieved succesfully.", body = [PropertySummary]),
        (status = 500, description = "Internal server error occurred."),
    )
)]
#[get("/boosted-properties")]
pub async fn fetch_boosted_properties(pool: web::Data<DbPool>) -> impl Responder {
    info!("request to get boosted properties");

    let result = web::block(move || {
        let conn_result = pool.get();
        match conn_result {
            Ok(mut conn) => PgProperties::fetch_top_properties(&mut conn),
            Err(e) => {
                error!("Failed to get connection from pool {}", e);
                vec![]
            }
        }
    })
    .await;

    match result {
        Ok(properties_list) => {
            let props_summary: Vec<PropertySummary> = properties_list
                .into_iter()
                .map(|p| PropertySummary {
                    id: p.property.id.to_string(),
                    image_path: p.property.img_path,
                    title: p.property.title,
                    street: p.street,
                    house_number: p.house_number,
                    neighborhood: p.neighborhood,
                    zip_code: p.zip_code,
                    city: p.city_name,
                    state: p.state_name,
                    price: p.property.price,
                    n_rooms: p.property.n_rooms,
                    n_bathrooms: p.property.n_bathrooms,
                    sqm: p.property.sqm,
                    property_type: p.property_type,
                    disposition: p.disposition,
                })
                .collect();

            HttpResponse::Ok().json(props_summary)
        }
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Fetches the details of a property by its ID.
#[get("/property/{id}")]
pub async fn fetch_property_details(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> impl Responder {
    info!("request to get property details");

    let result = web::block(move || {
        let conn_result = pool.get();
        match conn_result {
            Ok(mut conn) => {
                PgProperties::fetch_property_details(&mut conn, uuid::Uuid::from_str(&id).unwrap())
            }
            Err(e) => {
                error!("Failed to get connection from pool {}", e);
                None
            }
        }
    })
    .await;

    match result {
        Ok(Some(property_details)) => {
            let property_detail = PropertyDetail {
                id: property_details.property.id.to_string(),
                title: property_details.property.title,
                image_path: property_details.property.img_path,
                description: property_details
                    .property
                    .description
                    .unwrap_or(String::new()),
                n_rooms: property_details.property.n_rooms,
                n_bathrooms: property_details.property.n_bathrooms,
                sqm: property_details.property.sqm,
                price: property_details.property.price,
                street: property_details.street,
                owner_id: property_details.property.owner_id.to_string(),
                created_at: property_details.property.created_at.to_string(),
                house_number: property_details.house_number,
                neighborhood: property_details.neighborhood,
                zip_code: property_details.zip_code,
                city: property_details.city_name,
                state: property_details.state_name,
                property_type: property_details.property_type,
                disposition: property_details.disposition,
            };

            HttpResponse::Ok().json(property_detail)
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("Failed to get property details: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
