use crate::card::{Card, Identifier};
use crate::client::endpoints::cards::*;
use crate::client::endpoints::execute;
use crate::client::endpoints::sets::SetCode;
use crate::list::List;
use crate::Set;
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

    pub async fn card_search(&self, query: &str) -> SfResult<List<Card>> {
        let ep = Search::new(query);
        execute(ep, self).await
    }

    pub async fn card_named(&self, fuzzy: bool, query: &str) -> SfResult<Card> {
        let ep = Named::new(fuzzy, query);
        execute(ep, self).await
    }

    pub async fn card_autocomplete(&self, query: &str) -> SfResult<List<String>> {
        let ep = Autocomplete::new(query);
        execute(ep, self).await
    }

    pub async fn card_random(&self) -> SfResult<Card> {
        let ep = Random::new();
        execute(ep, self).await
    }

    pub async fn card_collections(&self, identifiers: Vec<Identifier>) -> SfResult<List<Card>> {
        let ep = Collection::new(identifiers);
        execute(ep, self).await
    }

    pub async fn card_set_and_collector_number(
        &self,
        set_code: String,
        collector_number: usize,
    ) -> SfResult<Card> {
        let ep = SetAndCollectorNumber::new(set_code, collector_number);
        execute(ep, self).await
    }

    pub async fn card_multiverse(&self, multiverse_id: usize) -> SfResult<Card> {
        let ep = Multiverse::new(multiverse_id);
        execute(ep, self).await
    }

    pub async fn card_mtgo(&self, mtgo_id: usize) -> SfResult<Card> {
        let ep = Mtgo::new(mtgo_id);
        execute(ep, self).await
    }

    pub async fn card_arena(&self, arena_id: usize) -> SfResult<Card> {
        let ep = Arena::new(arena_id);
        execute(ep, self).await
    }

    pub async fn card_tcgplayer(&self, tcgplayer_id: usize) -> SfResult<Card> {
        let ep = TcgPlayer::new(tcgplayer_id);
        execute(ep, self).await
    }

    pub async fn card_id(&self, scryfall_id: String) -> SfResult<Card> {
        let ep = ScryfallId::new(scryfall_id);
        execute(ep, self).await
    }

    pub async fn set_code(&self, set_code: String) -> SfResult<Set> {
        let ep = SetCode::new(set_code);
        execute(ep, self).await
    }

    pub async fn set_tcgplayer(&self, tcgplayer_id: usize) -> SfResult<Set> {
        let ep = crate::client::endpoints::sets::TcgPlayer::new(tcgplayer_id);
        execute(ep, self).await
    }

    pub async fn set_id(&self, scryfall_id: String) -> SfResult<Set> {
        let ep = crate::client::endpoints::sets::ScryfallId::new(scryfall_id);
        execute(ep, self).await
    }
}

impl Default for SfClient {
    fn default() -> Self {
        Self::new()
    }
}
