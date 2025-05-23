use std::str::FromStr;

use crate::dal::sch_models::{UpdateLocation, UpdateProperty};
use crate::dto::new_property::NewProperty;
use crate::dto::property_detail::PropertyDetail;
use crate::dto::property_preview::PropertyPreview;
use crate::dto::update_image_path::UpdateImagePath;
use crate::dto::update_property::UpdatedProperty;
use crate::dto::update_property_priority::UpdatePropertyPriority;
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
#[get("/properties")]
pub async fn fetch_properties(pool: web::Data<DbPool>) -> impl Responder {
    info!("request to get properties");

    let result = web::block(move || {
        let conn_result = pool.get();
        match conn_result {
            Ok(mut conn) => PgProperties::fetch_properties(&mut conn),
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
                    priority: p.priority,
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

/// Updates an existing property
#[put("/property/{id}")]
pub async fn update_property(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    updated_property_json: web::Json<UpdatedProperty>,
) -> impl Responder {
    info!("request to update property with ID: {}", id);

    let uuid = match uuid::Uuid::from_str(&id) {
        Ok(value) => value,
        Err(e) => {
            error!("Misformatted uuid: {}", e);
            return HttpResponse::BadRequest().finish();
        }
    };

    let property = updated_property_json.clone();
    let updated_property_sch = UpdateProperty {
        title: Some(property.title),
        img_path: Some(property.image_path),
        description: Some(property.description),
        n_rooms: Some(property.n_rooms),
        n_bathrooms: Some(property.n_bathrooms),
        sqm: Some(property.sqm),
        priority: Some(property.priority),
        price: Some(property.price),
        modified_at: Some(chrono::Utc::now().naive_utc()),
        property_type_id: Some(property.property_type_id),
        disposition_type_id: Some(property.disposition_type_id),
    };
    let updated_location_sch = UpdateLocation {
        street: Some(property.street),
        house_number: Some(property.house_number),
        neighborhood: Some(property.neighborhood),
        zip_code: Some(property.zip_code),
        latitude: Some(property.latitude),
        longitude: Some(property.longitude),
        city_name: Some(property.city_name),
        state_id: Some(property.state_id),
    };

    let result = web::block(move || match pool.get() {
        Ok(mut conn) => {
            match PgProperties::update_property_location_transaction(
                &mut conn,
                uuid,
                updated_property_sch,
                updated_location_sch,
            ) {
                Ok(_) => true,
                Err(e) => {
                    error!("Error while updating property and location: {}", e);
                    false
                }
            }
        }
        Err(e) => {
            error!("Error while getting database pool: {}", e);
            false
        }
    })
    .await;

    match result {
        Ok(true) => HttpResponse::Ok().finish(),
        Ok(false) => HttpResponse::Conflict().finish(),
        Err(e) => {
            error!("Couldn't update property: {}", e);
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
                modified_at: property_details.property.modified_at.to_string(),
                house_number: property_details.house_number,
                neighborhood: property_details.neighborhood,
                zip_code: property_details.zip_code,
                city: property_details.city_name,
                state: property_details.state_name,
                state_id: property_details.state_id,
                latitude: property_details.latitude,
                longitude: property_details.longitude,
                property_type: property_details.property_type,
                property_type_id: property_details.property_type_id,
                disposition: property_details.disposition,
                disposition_type_id: property_details.disposition_type_id,
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
pub async fn update_image_path(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    img_path: web::Json<UpdateImagePath>,
) -> HttpResponse {
    info!("Updating image path for property with ID: {}", id);

    let id = id.into_inner();
    let img_path = img_path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        PgProperties::update_image_path(
            &mut conn,
            Uuid::from_str(&id).unwrap(),
            img_path.image_path,
        )
        .unwrap();
    })
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Insert images array by property
#[post("/property-images/{id}")]
pub async fn insert_images_by_property(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    images: web::Json<Vec<crate::dto::new_image::NewImage>>,
) -> HttpResponse {
    info!(
        "Inserting image information array for property with ID: {}",
        id
    );

    let result_uuid = Uuid::from_str(&id.into_inner());
    let property_uuid = match result_uuid {
        Ok(uuid) => uuid,
        Err(_) => {
            error!("Invalid uuid format");
            return HttpResponse::BadRequest().finish();
        }
    };

    let sch_images_vec = images
        .clone()
        .into_iter()
        .map(|dto| crate::dal::sch_models::NewImage {
            id: uuid::Uuid::new_v4(),
            path: dto.path,
            name: dto.name,
            property_id: property_uuid,
        })
        .collect();

    let result = web::block(move || {
        let conn_result = pool.get();
        match conn_result {
            Ok(mut conn) => PgProperties::insert_images(&mut conn, sch_images_vec)
                .expect("Failed to get images from database"),
            Err(e) => {
                error!("Failed getting connection from pool: {}", e);
                0
            }
        }
    })
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Error deleting property: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Fetch images by property uuid
#[get("/property-images/{id}")]
pub async fn fetch_images_by_property(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> HttpResponse {
    info!("Fetching images of property with ID: {}", id);

    let result_uuid = Uuid::from_str(&id.into_inner());
    let property_uuid = match result_uuid {
        Ok(uuid) => uuid,
        Err(_) => {
            error!("Invalid uuid format");
            return HttpResponse::BadRequest().finish();
        }
    };

    let result = web::block(move || {
        let conn_result = pool.get();
        match conn_result {
            Ok(mut conn) => PgProperties::fetch_images(&mut conn, property_uuid)
                .expect("Failed to get images from database"),
            Err(e) => {
                error!("Failed getting connection from pool: {}", e);
                vec![]
            }
        }
    })
    .await;

    match result {
        Ok(db_images) => {
            let images_dto: Vec<crate::dto::image::Image> = db_images
                .into_iter()
                .map(|x| crate::dto::image::Image {
                    id: x.id.to_string(),
                    path: x.path,
                    name: x.name,
                })
                .collect();

            HttpResponse::Ok().json(images_dto)
        }
        Err(e) => {
            error!("Error fetching properties: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Delete a single image by its uuid
#[delete("/image/{id}")]
pub async fn delete_image_by_uuid(pool: web::Data<DbPool>, id: web::Path<String>) -> HttpResponse {
    info!("Deleting image with ID: {}", id);

    let result_uuid = Uuid::from_str(&id.into_inner());
    let image_uuid = match result_uuid {
        Ok(uuid) => uuid,
        Err(_) => {
            error!("Invalid uuid format");
            return HttpResponse::BadRequest().finish();
        }
    };

    let result = web::block(move || -> bool {
        let mut conn = pool.get().unwrap();
        let result = PgProperties::delete_image_by_uuid(&mut conn, image_uuid);

        match result {
            Ok(_) => return true,
            Err(e) => {
                error!("Could't delete all images by property: {}", e);
                return false;
            }
        }
    })
    .await;

    match result {
        Ok(success) => {
            if success {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            error!("Error deleting image: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Delete all images related to a property
#[delete("/property-images/{id}")]
pub async fn delete_all_property_images(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> HttpResponse {
    info!("Deleting all images for property with ID: {}", id);

    let result_uuid = Uuid::from_str(&id.into_inner());
    let property_uuid = match result_uuid {
        Ok(uuid) => uuid,
        Err(_) => {
            error!("Invalid uuid format");
            return HttpResponse::BadRequest().finish();
        }
    };

    let result = web::block(move || -> bool {
        let mut conn = pool.get().unwrap();
        let result = PgProperties::delete_all_images_by_property_uuid(&mut conn, property_uuid);

        match result {
            Ok(_) => return true,
            Err(e) => {
                error!("Could't delete all images by property: {}", e);
                return false;
            }
        }
    })
    .await;

    match result {
        Ok(success) => {
            if success {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            error!("Error deleting images: {}", e);
            HttpResponse::InternalServerError().finish()
        }
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
                    priority: property.priority,
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
        (status = 400, description = "Invalid uuid format"),
        (status = 404, description = "Property not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/property/{id}")]
pub async fn delete_property(pool: web::Data<DbPool>, id: web::Path<String>) -> HttpResponse {
    info!("Deleting property with ID: {}", id);

    let result_uuid = Uuid::from_str(&id.into_inner());
    let property_uuid = match result_uuid {
        Ok(uuid) => uuid,
        Err(_) => {
            error!("Invalid uuid format");
            return HttpResponse::BadRequest().finish();
        }
    };

    let result = web::block(move || -> bool {
        let mut conn = pool.get().unwrap();

        let result = PgProperties::delete_property_location_transaction(&mut conn, property_uuid);

        match result {
            Ok(_) => return true,
            Err(e) => {
                error!("Failed to delete property and its location: {}", e);
                return false;
            }
        }
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

#[put("/property-priority/{id}")]
async fn update_property_priority(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    priority: web::Json<UpdatePropertyPriority>,
) -> HttpResponse {
    info!("Updating property priority with ID: {}", id);

    let result = web::block(move || -> bool {
        let mut result = true;

        let mut conn = pool.get().unwrap();
        let property_id = Uuid::from_str(&id).unwrap();
        let property_update_result =
            PgProperties::update_property_priority(&mut conn, property_id, priority.0.new_priority);

        if property_update_result.is_err() {
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
            error!("Error updating property priority: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Fetches the preview of a property by its ID
#[get("/user-property-preview/{id}")]
pub async fn get_user_property(
    pool: web::Data<DbPool>,
    property_id: web::Path<String>,
) -> HttpResponse {
    info!("Getting user property preview");

    let result = web::block(move || {
        let conn = pool.get();
        match conn {
            Ok(mut conn) => {
                PgProperties::get_property_preview_by_property_id(&mut conn, uuid::Uuid::from_str(&property_id).unwrap())
            }
            Err(e) => {
                error!("Failed to get connection from pool {}", e);
                None
            }
        }
    })
    .await;

    match result {
        Ok(Some(property)) => {
            let property_preview = PropertyPreview {
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
                    priority: property.priority,
            };
                
            HttpResponse::Ok().json(property_preview)
        }

        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("Failed to get property preview: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}