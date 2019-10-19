use crate::client::endpoints::{Endpoint, Method};
use crate::Set;

pub struct ScryfallId {
    scryfall_id: String,
}

impl ScryfallId {
    pub fn new(scryfall_id: String) -> Self {
        Self { scryfall_id }
    }
}

impl Endpoint for ScryfallId {
    type Output = Set;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/sets"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn slash(&self) -> Option<String> {
        Some(format!("/{}", self.scryfall_id))
    }
}
