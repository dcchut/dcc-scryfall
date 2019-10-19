use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Core {
    pub arena_id: Option<usize>,
    pub id: String,
    pub lang: String,
    pub mtgo_id: Option<usize>,
    pub mtgo_foil_id: Option<usize>,
    pub multiverse_ids: Option<Vec<usize>>,
    pub tcgplayer_id: Option<usize>,
    pub oracle_id: String,
    pub prints_search_uri: String,
    pub rulings_uri: String,
    pub scryfall_uri: String,
    pub uri: String,
}
