use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdatePropertyPriority {
    pub new_priority: i32,
}
