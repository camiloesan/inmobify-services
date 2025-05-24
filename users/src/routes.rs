use crate::{
    dal::{db_operations::PgUsers, repository::UsersRepository},
    dto::new_user::NewUser,
    dto::update_user::UpdateUser,
    DbPool,
};
use actix_web::{
    delete, get, post, put, web::{self}, HttpResponse, Responder
};
//use actix_web_validator::Json;
use log::{error, info};
use validator::Validate;

/// Create a user based on its json DTO.
#[utoipa::path(
    responses(
        (status = 201, description = "User created successfully."),
        (status = 409, description = "User already exists."),
        (status = 500, description = "Internal server error occurred."),
    )
)]
#[post("/users")]
pub async fn create_user(pool: web::Data<DbPool>, data: web::Json<NewUser>) -> impl Responder {
    info!("request to create user received");

    if let Err(errors) = data.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let result = web::block(move || {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        PgUsers::create_user(data.into_inner(), &mut conn)
    })
    .await;

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
#[get("/users/{id}")]
pub async fn get_user_by_uuid(pool: web::Data<DbPool>, path: web::Path<String>) -> impl Responder {
    info!("request to get user received");

    let user_id = path.into_inner();
    let parsed_uuid = uuid::Uuid::parse_str(&user_id).expect("Uuid format not valid");

    let result = web::block(move || {
        let mut conn = pool.get().expect("failed to get connection");
        PgUsers::fetch_user_by_uuid(parsed_uuid, &mut conn)
    })
    .await;

    match result {
        Ok(Some(user_dto)) => HttpResponse::Ok().json(user_dto),
        Ok(None) => HttpResponse::Conflict().finish(),
        Err(e) => {
            error!("Couldn't fetch user: {}", e);
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
#[delete("/users/{id}")]
pub async fn delete_user_by_uuid(
    repo: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    info!("request to delete a user received");

    let user_id = path.into_inner();
    let parsed_uuid = uuid::Uuid::parse_str(&user_id).expect("bad format uuid");

    let result = web::block(move || {
        let mut conn = repo.get().expect("failed to get connection");
        PgUsers::delete_user_by_uuid(parsed_uuid, &mut conn)
    })
    .await;

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

/// Update a user by its uuid.
#[utoipa::path(
    responses(
        (status = 200, description = "User updated successfully."),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[put("/users/{id}")]
pub async fn update_user_by_uuid(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    updated_user: web::Json<UpdateUser>,
) -> impl Responder {
    info!("Request to update user received");

    if let Err(errors) = updated_user.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let user_id = path.into_inner();
    let parsed_uuid = uuid::Uuid::parse_str(&user_id).expect("bad format uiid");
    let updated_user = updated_user.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("failed to connect to database");
        PgUsers::update_user_by_uuid(parsed_uuid, updated_user, &mut conn)
    })
    .await;

   match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            error!("Error updating user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
