use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NewUser {
    #[schema(example = "Claudia", required = true)]
    pub name: String,
    #[schema(example = "Sheinbaum", required = true)]
    pub last_name: String,
    #[schema(example = "clauds@gmail.com", required = true)]
    pub email: String,
    #[schema(example = "1234567890", required = true)]
    pub phone: String,
    #[schema(example = "Sheinbaum", required = true)]
    pub password: String,
    #[schema(example = "1", required = true)]
    pub user_type_id: i32,
}
