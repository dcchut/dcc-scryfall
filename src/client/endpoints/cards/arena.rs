use crate::client::endpoints::{Endpoint, Method};
use crate::Card;

pub struct Arena {
    arena_id: usize,
}

impl Arena {
    pub fn new(arena_id: usize) -> Self {
        Self { arena_id }
    }
}

impl Endpoint for Arena {
    type Output = Card;

    fn uri(&self) -> &'static str {
        "https://api.scryfall.com/cards/arena"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn slash(&self) -> Option<String> {
        Some(format!("/{}", self.arena_id))
    }
}
