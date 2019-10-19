use crate::client::endpoints::{Endpoint, Method, Query};
use crate::Card;

pub struct Named<'a> {
    fuzzy: bool,
    query: &'a str,
}

impl<'a> Named<'a> {
    pub fn new(fuzzy: bool, query: &'a str) -> Self {
        Self { fuzzy, query }
    }
}

impl<'a> Endpoint for Named<'a> {
    type Output = Card;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/cards/named"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn query(&self) -> Vec<Query<'_, '_>> {
        vec![
            (if self.fuzzy { "fuzzy" } else { "exact" }, self.query),
            ("pretty", "true"),
        ]
    }
}
