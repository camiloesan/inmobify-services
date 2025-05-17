use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateImagePath {
    pub image_path: String,
}
