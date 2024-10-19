//! Dependencies to make HTTP requests
use log::trace;
use reqwest::blocking::Client;
use thiserror::Error;

/// Errors from the HTTP Provider
#[derive(Error, Debug, Clone, PartialEq)]
pub enum HTTPError {
    #[error("Missing required envvar: {0}")]
    MissingEnvVarError(String),

    #[error("Unable to fetch: {0}")]
    FetchError(String),

    #[error("Failed to parse JSON: {0}")]
    ParseError(String),
}

/// Trait to build and send HTTP Requests (not async)
pub trait HTTPProvider {
    /// Prepares a GET request to the specified endpoint
    fn get(&self, endpoint: &str) -> Result<String, HTTPError>;

    /// Adds a cookie that later will be used to fetch the data
    fn set_cookie(&mut self, cookie: String);

    /// Retrieves the stored cookie
    fn get_cookie(&self) -> Option<String>;
}

#[derive(Default)]
struct HTTPAdapter {
    aoc_cookie: Option<String>,
}

impl HTTPProvider for HTTPAdapter {
    fn get(&self, endpoint: &str) -> Result<String, HTTPError> {
        trace!("GET {}...", endpoint);
        let client = Client::default();
        let aoc_cookie = self.get_cookie().ok_or(
            HTTPError::MissingEnvVarError("AOC_COOKIE not set".into()),
        )?;

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

    fn set_cookie(&mut self, cookie: String) {
        self.aoc_cookie = Some(cookie);
    }

    fn get_cookie(&self) -> Option<String> {
        self.aoc_cookie.clone()
    }
}

pub fn get_default_http_provider() -> impl HTTPProvider {
    HTTPAdapter::default()
}
