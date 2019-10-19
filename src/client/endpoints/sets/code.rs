use crate::client::endpoints::{Endpoint, Method};
use crate::Set;

pub struct SetCode {
    set_code: String,
}

impl SetCode {
    pub fn new(set_code: String) -> Self {
        Self { set_code }
    }
}

impl Endpoint for SetCode {
    type Output = Set;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/sets"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn slash(&self) -> Option<String> {
        Some(format!("/{}", self.set_code))
    }
}
