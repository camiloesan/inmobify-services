use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Image {
    pub id: String,
    pub path: String,
    pub name: String,
}
