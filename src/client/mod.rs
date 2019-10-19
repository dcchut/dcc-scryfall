use crate::card::Card;
use crate::client::endpoints::{execute, Search};
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
}

impl Default for SfClient {
    fn default() -> Self {
        Self::new()
    }
}
