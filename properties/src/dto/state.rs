use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    pub id: i32,
    pub name: String,
}
