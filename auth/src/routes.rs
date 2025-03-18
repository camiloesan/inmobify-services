use actix_web::{post, web, HttpResponse, Responder};
use crate::dto::login::LoginRequest;
use crate::dal::db_operations::PgAuth;
use crate::dal::repository::AuthRepository;
use crate::DbPool;
use log::{error, info};
use jwt::generate_jwt;

#[utoipa::path(
    responses(
        (status = 200, description = "User authenticated successfully.", body = LoginResponse),
        (status = 401, description = "Unauthorized: Invalid credentials."),
        (status = 500, description = "Internal server error occurred."),
    )
)]

#[post("/login")]
pub async fn login(pool: web::Data<DbPool>, data: web::Json<LoginRequest>) -> impl Responder {
    info!("Request to login received");
    let login_data = data.into_inner();
    let result = web::block(move || {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        PgAuth::login(login_data, &mut conn)
    })
    .await;

    match result {
        Ok(Some((response, user_id))) => {
            match generate_jwt(user_id) {
                Ok(token) => {
                    info!("User authenticated successfully");
                    HttpResponse::Ok()
                        .insert_header(("x-token", token))
                        .json(response)
                }
                Err(e) => {
                    error!("Failed to generate JWT: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Ok(None) => {
            info!("Login failed: invalid credentials");
            HttpResponse::Unauthorized().finish()
        }
        Err(e) => {
            error!("Error during login: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}