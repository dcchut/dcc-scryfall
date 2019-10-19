use crate::card::price::Price;
use crate::imageuri::ImageUri;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Print {
    pub artist: Option<String>,
    pub booster: bool,
    pub border_color: String,
    pub card_back_id: String,
    pub collector_number: String,
    pub digital: bool,
    pub flavor_text: Option<String>,
    pub frame_effects: Option<Vec<String>>,
    pub frame: String,
    pub full_art: bool,
    pub games: Vec<String>,
    pub highres_image: bool,
    pub illustration_id: Option<String>,
    pub image_uris: Option<ImageUri>,
    pub prices: Price,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub promo: bool,
    pub promo_types: Option<Vec<String>>,
    pub purchase_uris: HashMap<String, String>,
    pub rarity: String,
    pub related_uris: HashMap<String, String>,
    pub released_at: String,
    pub reprint: bool,
    pub scryfall_set_uri: String,
    pub set_name: String,
    pub set_search_uri: String,
    pub set_type: String,
    pub set_uri: String,
    pub set: String,
    pub story_spotlight: bool,
    pub textless: bool,
    pub variation: bool,
    pub variation_of: Option<String>,
    pub watermark: Option<String>,
}
