//! Dependencies to make HTTP requests
use crate::constants;
use log::trace;
use reqwest::blocking::Client;
use std::env;
use thiserror::Error;

/// Errors from the HTTP Provider
#[derive(Error, Debug, Clone, PartialEq)]
pub enum HTTPError {
    #[error("The AOC cookie is missing: {0}")]
    MissingEnvVarError(#[from] std::env::VarError),

    #[error("Unable to fetch: {0}")]
    FetchError(String),

    #[error("Failed to parse JSON: {0}")]
    ParseError(String),
}

/// Trait to build and send HTTP Requests (not async)
pub trait HTTPProvider {
    /// Prepares a GET request to the specified endpoint
    fn get(&self, endpoint: &str) -> Result<String, HTTPError>;
}

#[derive(Default)]
struct HTTPAdapter {}

impl HTTPProvider for HTTPAdapter {
    fn get(&self, endpoint: &str) -> Result<String, HTTPError> {
        trace!("GET {}...", endpoint);
        let client = Client::default();
        let aoc_cookie = env::var(constants::AOC_COOKIE)?;

        let response = client
            .get(endpoint)
            .header(reqwest::header::COOKIE, aoc_cookie)
            .send()
            .map_err(|e| {
                HTTPError::FetchError(format!(
                    "GET {} failed: {}",
                    endpoint,
                    e.to_string()
                ))
            })?;

        let result = response
            .text()
            .map_err(|e| HTTPError::ParseError(e.to_string()))?;

        trace!("Response: {:?}", result);

        Ok(result)
    }
}

pub fn get_default_http_provider() -> impl HTTPProvider {
    HTTPAdapter::default()
}
