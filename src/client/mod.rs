use crate::card::{Card, Identifier};
use crate::client::endpoints::*;
use crate::list::List;
use reqwest::Client as ReqClient;

mod endpoints;

pub type SfResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct SfClient {
    inner: ReqClient,
}

impl SfClient {
    pub fn from_client(inner: ReqClient) -> Self {
        Self { inner }
    }

    pub fn new() -> Self {
        Self {
            inner: ReqClient::new(),
        }
    }

    pub async fn search(&self, query: &str) -> SfResult<List<Card>> {
        let ep = Search::new(query);
        execute(ep, self).await
    }

    pub async fn named(&self, fuzzy: bool, query: &str) -> SfResult<Card> {
        let ep = Named::new(fuzzy, query);
        execute(ep, self).await
    }

    pub async fn autocomplete(&self, query: &str) -> SfResult<List<String>> {
        let ep = Autocomplete::new(query);
        execute(ep, self).await
    }

    pub async fn random(&self) -> SfResult<Card> {
        let ep = Random::new();
        execute(ep, self).await
    }

    pub async fn collections(&self, identifiers: Vec<Identifier>) -> SfResult<List<Card>> {
        let ep = Collection::new(identifiers);
        execute(ep, self).await
    }

    pub async fn set_and_collector_number(&self, set_code: String, collector_number: usize) -> SfResult<Card> {
        let ep = SetAndCollectorNumber::new(set_code, collector_number);
        execute(ep, self).await
    }

    pub async fn multiverse(&self, multiverse_id: usize) -> SfResult<Card> {
        let ep = Multiverse::new(multiverse_id);
        execute(ep, self).await
    }

    pub async fn mtgo(&self, mtgo_id: usize) -> SfResult<Card> {
        let ep = Mtgo::new(mtgo_id);
        execute(ep, self).await
    }

    pub async fn arena(&self, arena_id: usize) -> SfResult<Card> {
        let ep = Arena::new(arena_id);
        execute(ep, self).await
    }

    pub async fn tcgplayer(&self, tcgplayer_id: usize) -> SfResult<Card> {
        let ep = TcgPlayer::new(tcgplayer_id);
        execute(ep, self).await
    }

    pub async fn id(&self, scryfall_id: String) -> SfResult<Card> {
        let ep = ScryfallId::new(scryfall_id);
        execute(ep, self).await
    }
}

impl Default for SfClient {
    fn default() -> Self {
        Self::new()
    }
}
