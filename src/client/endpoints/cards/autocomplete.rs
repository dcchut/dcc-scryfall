use crate::client::endpoints::{Endpoint, Method, Query};
use crate::List;

pub struct Autocomplete<'a> {
    query: &'a str,
}

impl<'a> Autocomplete<'a> {
    pub fn new(query: &'a str) -> Self {
        Self { query }
    }
}

impl<'a> Endpoint for Autocomplete<'a> {
    type Output = List<String>;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/cards/autocomplete"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn query(&self) -> Vec<Query<'_, '_>> {
        vec![("q", self.query)]
    }
}
