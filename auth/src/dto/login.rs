use serde::{Serialize, Deserialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]

pub struct LoginRequest {
    #[schema(example = "juan@gmail.com", required = true)]
    pub email: String,
    pub password: String,
}