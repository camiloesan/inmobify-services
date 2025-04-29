use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateImagePath {
    pub img_path: String,
}
