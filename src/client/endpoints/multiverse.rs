use crate::client::endpoints::{Endpoint, Method};
use crate::Card;

pub struct Multiverse {
    multiverse_id: usize,
}

impl Multiverse {
    pub fn new(multiverse_id: usize) -> Self {
        Self { multiverse_id }
    }
}

impl Endpoint for Multiverse {
    type Output = Card;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/cards/multiverse"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn slash(&self) -> Option<String> {
        Some(format!("/{}", self.multiverse_id))
    }
}
