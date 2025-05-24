use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

fn load_env() {
    let first_try = dotenvy::dotenv();
    if first_try.is_err() {
        dotenvy::from_path(std::path::Path::new("auth/.env")).expect("dotenvy failed");
    }
}

pub fn generate_jwt(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    load_env();
    let secret_key = std::env::var("JWT_SECRET").expect("JWT_SECRET not in .env");

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_secs() as usize
        + 7200; // Expira en 2 horas

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
}

pub async fn validate_jwt(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    load_env();
    let secret_key = std::env::var("JWT_SECRET").expect("JWT_SECRET not in .env");

    let token = credentials.token();

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default(),
    ) {
        Ok(_) => Ok(req),
        Err(_) => Err(ErrorUnauthorized("Invalid token")),
    }
}
