use crate::{SfClient, SfResult};
pub use autocomplete::Autocomplete;
pub use named::Named;
pub use search::Search;
pub use random::Random;
pub use set_collector::SetAndCollectorNumber;
pub use collection::Collection;
pub use multiverse::Multiverse;
pub use arena::Arena;
pub use mtgo::Mtgo;
pub use tcgplayer::TcgPlayer;
pub use id::ScryfallId;

use serde::de::DeserializeOwned;

mod arena;
mod autocomplete;
mod id;
mod mtgo;
mod multiverse;
mod named;
mod search;
mod random;
mod collection;
mod set_collector;
mod tcgplayer;

#[derive(Copy, Clone)]
pub enum Method {
    POST,
    GET,
}

pub type Query<'a, 'b> = (&'a str, &'b str);

pub trait Endpoint {
    /// The output associated with this API endpoint
    type Output;

    /// Returns the URI associated with this API endpoint
    fn uri(&self) -> &'static str;

    /// Returns any slash-based query parameters
    fn slash(&self) -> Option<String> {
        None
    }

    /// Returns the HTTP Method associated with this API endpoint
    fn method(&self) -> Method;

    /// Returns a list of query parameters for this API endpoint
    fn query(&self) -> Vec<Query<'_, '_>> {
        Vec::new()
    }

    /// The request body
    fn body(&self) -> Option<serde_json::Value> {
        None
    }
}

/// Executes the given endpoint on the provided client.
pub async fn execute<T: DeserializeOwned>(
    ep: impl Endpoint<Output = T>,
    client: &SfClient,
) -> SfResult<T> {
    let uri = match ep.slash() {
        Some(slashes) => format!("{}{}", ep.uri(), slashes),
        None => String::from(ep.uri()),
    };

    let mut req = match ep.method() {
        Method::GET => client.inner.get(&uri),
        Method::POST => client.inner.post(&uri),
    };

    // Add in any query parameters
    let queries = ep.query();

    if !queries.is_empty() {
        req = req.query(&queries);
    }

    // Add the request body
    match ep.body() {
        Some(value) => { req = req.json(&value); },
        None => {},
    }

    // Send the request
    let response = req.send().await?;

    // If the response is a 404, return an error
    if response.status() == 404 {
        return Err("Request returned status code 404".into());
    }

    let res = response.json().await?;

    Ok(res)
}
