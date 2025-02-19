use crate::{
    dal::{db_operations::PgUsers, repository::UsersRepository},
    dto::new_user::NewUser,
};
use actix_web::{
    delete, get, post,
    web::{self},
    HttpResponse, Responder,
};
use log::{error, info};

/// Create a user based on its json DTO.
#[utoipa::path(
    responses(
        (status = 201, description = "User created successfully."),
        (status = 409, description = "User already exists."),
        (status = 500, description = "Internal server error occurred."),
    )
)]
#[post("/user")]
pub async fn create_user(repo: web::Data<PgUsers>, data: web::Json<NewUser>) -> impl Responder {
    info!("request to create user received");

    let result = web::block(move || repo.create_user(data.0.clone())).await;
    match result {
        Ok(Some(uuid)) => HttpResponse::Created().json(uuid),
        Ok(None) => HttpResponse::Conflict().finish(),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Get a user by its uuid.
#[utoipa::path(
    responses(
        (status = 200, description = "User found.", body = User),
        (status = 500, description = "Internal server error occurred."),
    )
)]
#[get("/user/{id}")]
pub async fn get_user_by_uuid(repo: web::Data<PgUsers>, path: web::Path<String>) -> impl Responder {
    info!("request to get user received");

    let user_id = path.into_inner();
    let parsed_uuid = uuid::Uuid::parse_str(&user_id).expect("bad format uuid");

    let result = web::block(move || repo.get_user_by_uuid(parsed_uuid)).await;
    match result {
        Ok(Some(uuid)) => HttpResponse::Ok().json(uuid),
        Ok(None) => HttpResponse::Conflict().finish(),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Delete a user by its uuid.
#[utoipa::path(
    responses(
        (status = 200, description = "Deleted succesfully."),
        (status = 500, description = "Internal server error occurred."),
    )
)]
#[delete("/user/{id}")]
pub async fn delete_user_by_uuid(
    repo: web::Data<PgUsers>,
    path: web::Path<String>,
) -> impl Responder {
    info!("request to delete a user received");

    let user_id = path.into_inner();
    let parsed_uuid = uuid::Uuid::parse_str(&user_id).expect("bad format uuid");

    let result = web::block(move || repo.delete_user_by_uuid(parsed_uuid)).await;
    match result {
        Ok(is_deleted) => {
            if is_deleted {
                return HttpResponse::Ok().finish();
            }
            HttpResponse::Conflict().finish()
        }
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
