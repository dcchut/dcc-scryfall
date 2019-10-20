use serde::{Deserialize, Serialize};

use crate::color::Color;
use crate::imageuri::ImageUri;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Face {
    pub artist: Option<String>,
    pub color_indicator: Option<Vec<Color>>,
    pub colors: Option<Vec<Color>>,
    pub flavor_text: Option<String>,
    pub illustration_id: Option<String>,
    pub image_uris: Option<ImageUri>,
    pub loyalty: Option<String>,
    pub mana_cost: String,
    pub name: String,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub toughness: Option<String>,
    pub type_line: String,
    pub watermark: Option<String>,
}
