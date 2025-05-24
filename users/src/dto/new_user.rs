use serde::{Serialize, Deserialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct NewUser {
    #[schema(example = "Claudia", required = true)]
    #[validate(length(min = 1, max = 255, message = "El campo nombre es obligatorio"))]
    pub name: String,

    #[schema(example = "Sheinbaum", required = true)]
    #[validate(length(min = 1, max = 255, message = "El campo apellido es obligatorio"))]
    pub last_name: String,
    
    #[schema(example = "clauds@gmail.com", required = true)]
    #[validate(email(message = "El campo email debe ser un correo válido"))]
    pub email: String,

    #[schema(example = "1234567890", required = true)]
    #[validate(length(min = 10, max = 10, message = "El campo teléfono es obligatorio"))]
    pub phone: String,

    #[schema(example = "Sheinbaum", required = true)]
    #[validate(length(min = 1, max = 255, message = "El campo contraseña es obligatorio"))]
    pub password: String,
}