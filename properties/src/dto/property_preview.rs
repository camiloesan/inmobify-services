use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PropertyPreview {
    pub id: String,
    pub title: String,
    pub location: String,
    pub priority: i32,
}
