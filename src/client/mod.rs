use crate::card::Card;
use crate::client::endpoints::SEARCH_ENDPOINT;
use crate::list::List;
use reqwest::Client as ReqClient;

mod endpoints;

pub type SfResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct SfClient {
    inner: ReqClient,
}

impl SfClient {
    pub fn new() -> Self {
        Self {
            inner: ReqClient::new(),
        }
    }

    pub async fn search(&self, query: &str) -> SfResult<List<Card>> {
        let js: List<Card> = self
            .inner
            .get(SEARCH_ENDPOINT)
            .query(&[("q", query)])
            .send()
            .await?
            .json()
            .await?;

        Ok(js)
    }
}

impl Default for SfClient {
    fn default() -> Self {
        Self::new()
    }
}