use crate::{SfClient, SfResult};
use serde::de::DeserializeOwned;

pub mod cards;
pub mod sets;

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
    if let Some(value) = ep.body() {
        req = req.json(&value);
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
