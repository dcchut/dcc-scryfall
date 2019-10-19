use crate::client::endpoints::{Endpoint, Method};
use crate::Card;

pub struct ScryfallId {
    scryfall_id: String,
}

impl ScryfallId {
    pub fn new(scryfall_id: String) -> Self {
        Self { scryfall_id }
    }
}

impl Endpoint for ScryfallId {
    type Output = Card;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/cards"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn slash(&self) -> Option<String> {
        Some(format!("/{}", self.scryfall_id))
    }
}
