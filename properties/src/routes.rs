use std::str::FromStr;

use crate::dto::new_property::NewProperty;
use crate::dto::property_detail::PropertyDetail;
use crate::dto::update_image_path::UpdateImagePath;
use crate::DbPool;
use crate::{
    dal::{db_operations::PgProperties, repository::PropertiesRepository},
    dto::property_summary::PropertySummary,
};
use actix_web::{delete, post, put};
use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};
use log::{error, info};
use uuid::Uuid;

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

/// Create a new property.
#[post("/property")]
pub async fn create_property(
    pool: web::Data<DbPool>,
    new_property: web::Json<NewProperty>,
) -> impl Responder {
    info!("request to create property received");

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        let location = crate::dal::sch_models::NewLocation {
            street: &new_property.street,
            house_number: &new_property.house_number,
            neighborhood: &new_property.neighborhood,
            zip_code: &new_property.zip_code,
            latitude: &new_property.latitude,
            longitude: &new_property.longitude,
            city_name: &new_property.city_name,
            state_id: new_property.state_id,
        };
        let location_id = PgProperties::create_location(&mut conn, location).unwrap();

        let property = crate::dal::sch_models::NewProperty {
            id: Uuid::new_v4(),
            title: &new_property.title,
            img_path: &new_property.image_path,
            description: Some(&new_property.description),
            n_rooms: new_property.n_rooms,
            n_bathrooms: new_property.n_bathrooms,
            sqm: new_property.sqm,
            priority: new_property.priority,
            price: new_property.price,
            owner_id: Uuid::from_str(&new_property.owner_id).unwrap(),
            location_id,
            property_type_id: new_property.property_type_id,
            disposition_type_id: new_property.disposition_type_id,
        };
        let property_uuid = PgProperties::create_property(&mut conn, property).unwrap();

        property_uuid
    })
    .await;

    match result {
        Ok(property_uuid) => HttpResponse::Ok().json(property_uuid.to_string()),
        Err(err) => {
            error!("Failed to create property: {}", err);
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
                latitude: property_details.latitude,
                longitude: property_details.longitude,
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

/// Update the image path of a property
#[put("/property-img-path/{id}")]
pub async fn update_img_path(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    img_path: web::Json<UpdateImagePath>,
) -> HttpResponse {
    info!("Updating image path for property with ID: {}", id);

    let id = id.into_inner();
    let img_path = img_path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        PgProperties::update_image_path(&mut conn, Uuid::from_str(&id).unwrap(), img_path.img_path)
            .unwrap();
    })
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Get all states
#[get("/states")]
pub async fn get_states(pool: web::Data<DbPool>) -> HttpResponse {
    info!("Getting states");

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        PgProperties::get_states(&mut conn).unwrap()
    })
    .await;

    match result {
        Ok(states) => {
            let dto = states
                .into_iter()
                .map(|state| crate::dto::state::State {
                    id: state.id,
                    name: state.name,
                })
                .collect::<Vec<_>>();
            HttpResponse::Ok().json(dto)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/user-properties-preview/{id}")]
pub async fn get_user_properties(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
) -> HttpResponse {
    info!("Getting user properties");

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        PgProperties::get_top_5_properties_by_user_id(&mut conn, Uuid::from_str(&user_id).unwrap())
            .unwrap()
    })
    .await;

    match result {
        Ok(properties) => {
            let dto = properties
                .into_iter()
                .map(|property| crate::dto::property_preview::PropertyPreview {
                    id: property.id.to_string(),
                    title: property.title,
                    location: format!(
                        "{} {} {} {} {} {}",
                        property.street,
                        property.house_number,
                        property.neighborhood,
                        property.city_name,
                        property.state_name,
                        property.zip_code,
                    ),
                })
                .collect::<Vec<_>>();
            HttpResponse::Ok().json(dto)
        }
        Err(e) => {
            error!("Error getting user properties: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Deletes a property by ID.
#[utoipa::path(
    delete,
    path = "/property/{id}",
    responses(
        (status = 200, description = "Property deleted successfully"),
        (status = 404, description = "Property not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/property/{id}")]
pub async fn delete_property(pool: web::Data<DbPool>, id: web::Path<String>) -> HttpResponse {
    info!("Deleting property with ID: {}", id);

    let result = web::block(move || -> bool {
        let mut result = true;

        let mut conn = pool.get().unwrap();

        let location_id =
            PgProperties::get_location_id_by_property_uuid(&mut conn, Uuid::from_str(&id).unwrap())
                .unwrap();

        let property_deletion_result =
            PgProperties::delete_property_by_uuid(&mut conn, Uuid::from_str(&id).unwrap());

        if property_deletion_result.unwrap() > 0 {
            PgProperties::delete_location_by_id(&mut conn, location_id).unwrap();
        } else {
            result = false;
        }

        result
    })
    .await;

    match result {
        Ok(exists) => {
            if exists {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            error!("Error deleting property: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
