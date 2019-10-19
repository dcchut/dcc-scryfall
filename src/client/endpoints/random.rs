use crate::client::endpoints::{Endpoint, Method};
use crate::Card;

pub struct Random {}

impl Random {
    pub fn new() -> Self {
        Self {}
    }
}

impl Endpoint for Random {
    type Output = Card;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/cards/random"
    }

    fn method(&self) -> Method {
        Method::GET
    }
}
