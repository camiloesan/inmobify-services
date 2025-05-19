use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateUser {
    #[schema(example = "Claudia", required = false)]
    #[validate(length(min = 1, max = 255, message = "El campo nombre es obligatorio"))]
    pub name: Option<String>,

    #[schema(example = "Sheinbaum", required = false)]
    #[validate(length(min = 1, max = 255, message = "El campo apellido es obligatorio"))]
    pub last_name: Option<String>,

    #[schema(example = "clauds@gmail.com", required = false)]
    #[validate(email(message = "El campo email debe ser un correo válido"))]
    pub email: Option<String>,

    #[schema(example = "1234567890", required = false)]
    #[validate(length(min = 10, max = 10, message = "El campo teléfono es obligatorio"))]
    pub phone: Option<String>,

    #[schema(example = "Sheinbaum", required = false)]
    #[validate(length(min = 1, max = 255, message = "El campo contraseña es obligatorio"))]
    pub password: Option<String>
}
