use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]

pub struct LoginResponse {
    #[schema(example = "699f69e5-a2a4-4168-a535-b900a1c822be", required = true)]
    pub id: String,
    #[schema(example = "Claudia", required = true)]
    pub name: String,
    #[schema(example = "Sheinbaum", required = true)]
    pub last_name: String,
    #[schema(example = "clauds@gmail.com", required = true)]
    pub email: String,
    #[schema(example = "1234567890", required = true)]
    pub phone: String,
    #[schema(example = "1", required = true)]
    pub user_type_id: i32,
}