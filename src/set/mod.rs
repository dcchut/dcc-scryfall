use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Set {
    id: String,
    code: String,
    mtgo_code: String,
    tcgplayer_id: Option<usize>,
    name: String,
    set_type: String,
    released_at: Option<String>,
    block_code: Option<String>,
    block: Option<String>,
    parent_set_code: String,
    card_count: usize,
    digital: bool,
    foil_only: bool,
    scryfall_uri: String,
    uri: String,
    icon_svg_uri: String,
    search_uri: String,
}
