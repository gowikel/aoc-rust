//! Dependencies to make HTTP requests
use crate::Puzzle;
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
    fn get(&self, endpoint: &impl AOCUrl) -> Result<String, HTTPError>;

    /// Adds a cookie that later will be used to fetch the data
    fn set_cookie(&mut self, cookie: String);

    /// Retrieves the stored cookie
    fn get_cookie(&self) -> Option<String>;
}

#[derive(Default)]
pub struct HTTPAdapter {
    aoc_cookie: Option<String>,
}

/// This encapsulates endpoints to AOC website
pub trait AOCUrl {
    /// Fetch the required URL
    fn url(&self) -> String;
}

/// AOCUrl implementation
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct URL {
    url: String,
}

impl URL {
    /// Creates a new AOC Url from the provided puzzle
    pub fn new(puzzle: &Puzzle) -> Self {
        let endpoint = format!(
            "https://adventofcode.com/{}/day/{}/input",
            puzzle.year(),
            puzzle.day()
        );
        Self { url: endpoint }
    }
}

impl AOCUrl for URL {
    fn url(&self) -> String {
        self.url.clone()
    }
}

impl HTTPProvider for HTTPAdapter {
    fn get(&self, endpoint: &impl AOCUrl) -> Result<String, HTTPError> {
        trace!("GET {:?}...", endpoint.url());
        let client = Client::default();
        let endpoint = endpoint.url();
        let aoc_cookie = self.get_cookie().ok_or(
            HTTPError::MissingEnvVarError("AOC_COOKIE not set".into()),
        )?;

        let response = client
            .get(endpoint.clone())
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
            .error_for_status()
            .map_err(|e| HTTPError::FetchError(format!("{}", e)))?
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;

    pub struct HttpProviderMock {
        responses: HashMap<String, Result<String, HTTPError>>,
        cookie: Option<String>,
        calls: RefCell<Vec<String>>,
        assert_called_with: Option<String>,
        assert_cookie_value: Option<String>,
    }

    impl HTTPProvider for HttpProviderMock {
        fn get(&self, endpoint: &impl AOCUrl) -> Result<String, HTTPError> {
            self.calls.borrow_mut().push(endpoint.url());
            self.responses.get(&endpoint.url()).unwrap().clone()
        }

        fn set_cookie(&mut self, cookie: String) {
            self.cookie = Some(cookie);
        }

        fn get_cookie(&self) -> Option<String> {
            unimplemented!("`get_cookie` is not needed in the mocks`")
        }
    }

    impl HttpProviderMock {
        pub fn new() -> Self {
            Self {
                responses: HashMap::new(),
                cookie: None,
                calls: RefCell::new(Vec::new()),
                assert_called_with: None,
                assert_cookie_value: None,
            }
        }
        pub fn insert_response(&mut self, endpoint: String, response: String) {
            self.responses.insert(endpoint, Ok(response));
        }

        pub fn insert_error(&mut self, endpoint: String, error: HTTPError) {
            self.responses.insert(endpoint, Err(error));
        }

        pub fn assert_called_with(&mut self, endpoint: String) {
            self.assert_called_with = Some(endpoint);
        }

        pub fn assert_cookie_value(&mut self, endpoint: String) {
            self.assert_cookie_value = Some(endpoint);
        }
    }

    impl Drop for HttpProviderMock {
        fn drop(&mut self) {
            if let Some(expected) = &self.assert_called_with {
                assert!(
                    self.calls.borrow().contains(&expected),
                    "Expected call to {} not found. Calls: {:?}",
                    expected,
                    self.calls.borrow()
                );
            }

            if let Some(expected) = &self.assert_cookie_value {
                assert!(
                    self.cookie.is_some(),
                    "Cookie not set, but it was expected"
                );
                assert_eq!(self.cookie.clone().unwrap().as_str(), expected);
            }
        }
    }
}
