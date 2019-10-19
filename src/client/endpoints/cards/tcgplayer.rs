use crate::client::endpoints::{Endpoint, Method};
use crate::Card;

pub struct TcgPlayer {
    tcgplayer_id: usize,
}

impl TcgPlayer {
    pub fn new(tcgplayer_id: usize) -> Self {
        Self { tcgplayer_id }
    }
}

impl Endpoint for TcgPlayer {
    type Output = Card;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/cards/tcgplayer"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn slash(&self) -> Option<String> {
        Some(format!("/{}", self.tcgplayer_id))
    }
}
