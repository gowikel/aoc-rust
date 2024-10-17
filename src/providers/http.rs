//! Dependencies to make HTTP requests

use crate::constants;
use anyhow::{Context, Result};
use log::trace;
use reqwest::blocking::Client;
use std::env;

/// Trait to build and send HTTP Requests (not async)
pub trait HTTPProvider {
    /// Prepares a GET request to the specified endpoint
    fn get(&self, endpoint: &str) -> Result<String>;
}

#[derive(Default)]
struct HTTPAdapter {}

impl HTTPProvider for HTTPAdapter {
    fn get(&self, endpoint: &str) -> Result<String> {
        trace!("GET {}...", endpoint);
        let client = Client::default();
        let aoc_cookie =
            env::var(constants::AOC_COOKIE).with_context(|| {
                format!("Missing {} env variable", constants::AOC_COOKIE)
            })?;

        let response = client
            .get(endpoint)
            .header(reqwest::header::COOKIE, aoc_cookie)
            .send()
            .with_context(|| format!("Unable to GET {}", endpoint))?;

        let result = response.text().with_context(|| {
            format!("Unable to parse response from {}", endpoint)
        })?;

        trace!("Response: {:?}", result);

        Ok(result)
    }
}

pub fn get_default_http_providerr() -> impl HTTPProvider {
    HTTPAdapter::default()
}
