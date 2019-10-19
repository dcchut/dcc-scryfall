use crate::client::endpoints::{Endpoint, Method};
use crate::Card;

pub struct Mtgo {
    mtgo_id: usize,
}

impl Mtgo {
    pub fn new(mtgo_id: usize) -> Self {
        Self { mtgo_id }
    }
}

impl Endpoint for Mtgo {
    type Output = Card;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/cards/mtgo"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn slash(&self) -> Option<String> {
        Some(format!("/{}", self.mtgo_id))
    }
}
