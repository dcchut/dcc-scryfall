use crate::client::endpoints::{Endpoint, Method};
use crate::Card;

pub struct SetAndCollectorNumber {
    set_code: String,
    collector_number: usize,
    lang: Option<String>,
}

impl SetAndCollectorNumber {
    pub fn new(set_code: String, collector_number: usize) -> Self {
        Self {
            set_code,
            collector_number,
            lang: None,
        }
    }
}


impl Endpoint for SetAndCollectorNumber {
    type Output = Card;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/cards"
    }

    fn slash(&self) -> Option<String> {
        Some(match &self.lang {
            Some(lang) => format!("/{}/{}/{}", self.set_code, self.collector_number, lang),
            None => format!("/{}/{}", self.set_code, self.collector_number),
        })
    }

    fn method(&self) -> Method {
        Method::GET
    }
}
