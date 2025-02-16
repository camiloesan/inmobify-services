use crate::{
    dal::{db_operations::PgUsers, repository::UsersRepository}, dto_models::{NewUser, User},
};
use actix_web::{post, web, HttpResponse, Responder};
use log::{error, info};

/// Create an user based on its json DTO.
#[utoipa::path(
    request_body = Organization,
    responses(
        (status = 201, description = "User created successfully."),
        (status = 409, description = "User already exists."),
        (status = 500, description = "Internal server error occurred."),
    )
)]
#[post("/user")]
pub async fn create_user(
    repo: web::Data<PgUsers>,
    data: web::Json<NewUser>,
) -> impl Responder {
    info!("request to create organization received");

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

/// Create an user based on its json DTO.
#[utoipa::path(
    request_body = Organization,
    responses(
        (status = 201, description = "User created successfully."),
        (status = 409, description = "User already exists."),
        (status = 500, description = "Internal server error occurred."),
    )
)]
#[get("/user")]
pub async fn get_user_by_uuid(
    repo: web::Data<PgUsers>,
    data: web::Json<NewUser>,
) -> impl Responder {
    info!("request to create organization received");

    let result = web::block(move || repo.get_user_by_uuid(data.0.clone())).await;
    match result {
        Ok(Some(uuid)) => HttpResponse::Created().json(uuid),
        Ok(None) => HttpResponse::Conflict().finish(),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
