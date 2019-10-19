use crate::client::endpoints::{Endpoint, Method};
use crate::Set;

pub struct TcgPlayer {
    tcgplayer_id: usize,
}

impl TcgPlayer {
    pub fn new(tcgplayer_id: usize) -> Self {
        Self { tcgplayer_id }
    }
}

impl Endpoint for TcgPlayer {
    type Output = Set;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/sets/tcgplayer"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn slash(&self) -> Option<String> {
        Some(format!("/{}", self.tcgplayer_id))
    }
}
