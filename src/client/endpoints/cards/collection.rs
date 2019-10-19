use crate::card::Identifier;
use crate::client::endpoints::{Endpoint, Method};
use crate::{Card, List};
use serde_json::json;

pub struct Collection {
    identifiers: Vec<Identifier>,
}

impl Collection {
    pub fn new(identifiers: Vec<Identifier>) -> Self {
        Self { identifiers }
    }
}

impl Endpoint for Collection {
    type Output = List<Card>;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/cards/collection"
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn body(&self) -> Option<serde_json::Value> {
        Some(json!({
        "identifiers": self.identifiers.iter().map(|x| x.as_value()).collect::<Vec<_>>(),
        }))
    }
}
