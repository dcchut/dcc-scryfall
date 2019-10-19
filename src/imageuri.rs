use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUri {
    pub small: String,
    pub normal: String,
    pub large: String,
    pub png: String,
    pub art_crop: String,
    pub border_crop: String,
}
