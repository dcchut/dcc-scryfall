use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Set {
    pub id: String,
    pub code: String,
    pub mtgo_code: String,
    pub tcgplayer_id: Option<usize>,
    pub name: String,
    pub set_type: String,
    pub released_at: Option<String>,
    pub block_code: Option<String>,
    pub block: Option<String>,
    pub parent_set_code: Option<String>,
    pub card_count: usize,
    pub digital: bool,
    pub foil_only: bool,
    pub scryfall_uri: String,
    pub uri: String,
    pub icon_svg_uri: String,
    pub search_uri: String,
}
