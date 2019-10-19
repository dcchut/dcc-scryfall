use crate::{SfClient, SfResult};
pub use autocomplete::Autocomplete;
pub use named::Named;
pub use search::Search;

use serde::de::DeserializeOwned;

mod autocomplete;
mod named;
mod search;

#[derive(Copy, Clone)]
#[allow(dead_code)]
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

    /// Returns the HTTP Method associated with this API endpoint
    fn method(&self) -> Method;

    /// Returns a list of query parameters for this API endpoint
    fn query(&self) -> Vec<Query<'_, '_>> {
        Vec::new()
    }
}

/// Executes the given endpoint on the provided client.
pub async fn execute<T: DeserializeOwned>(
    ep: impl Endpoint<Output = T>,
    client: &SfClient,
) -> SfResult<T> {
    let mut req = match ep.method() {
        Method::GET => client.inner.get(ep.uri()),
        Method::POST => client.inner.post(ep.uri()),
    };

    // Add in any query parameters
    let queries = ep.query();

    if !queries.is_empty() {
        req = req.query(&queries);
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
