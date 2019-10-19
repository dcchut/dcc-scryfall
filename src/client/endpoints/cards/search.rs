use crate::client::endpoints::{Endpoint, Method, Query};
use crate::{Card, List};

pub struct Search<'a> {
    query: &'a str,
}

impl<'a> Search<'a> {
    pub fn new(query: &'a str) -> Self {
        Self { query }
    }
}

impl<'a> Endpoint for Search<'a> {
    type Output = List<Card>;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/cards/search"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn query(&self) -> Vec<Query<'_, '_>> {
        vec![("q", self.query)]
    }
}
