use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewImage {
    pub path: String,
    pub name: String,
}
