use crate::DbPool;
use actix_web::{
    get, web::{self}, HttpResponse, Responder
};
use log::{error, info};
use crate::{
    dal::{db_operations::PgProperties, repository::PropertiesRepository},
    dto::property_summary::PropertySummary,
};

/// Get a list of boosted properties.
#[utoipa::path(
    responses(
        (status = 200, description = "Retrieved succesfully.", body = [PropertySummary]),
        (status = 500, description = "Internal server error occurred."),
    )
)]
#[get("/boosted-properties")]
pub async fn fetch_boosted_properties(
    pool: web::Data<DbPool>,
) -> impl Responder {
    info!("request to get boosted properties");

    let result = web::block(move || {
        let conn_result = pool.get();
        match conn_result {
            Ok(mut conn) => PgProperties::fetch_top_properties(&mut conn),
            Err(e) => {
                error!("Failed to get connection from pool {}", e);
                vec![]
            },
        }
    })
    .await;

    match result {
        Ok(properties_list) => {
            let props_summary: Vec<PropertySummary> = properties_list
                .into_iter()
                .map(|p| PropertySummary {
                    id: p.property.id.to_string(),
                    image_path: "/assets/default.jpg".to_string(), //placeholder for path
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
