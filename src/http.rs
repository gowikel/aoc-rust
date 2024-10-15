//! Dependencies to make HTTP requests

use anyhow::Result;

/// Trait to build and send HTTP Requests (not async)
pub trait HTTPProvider {
    /// Prepares a GET request to the specified endpoint
    fn get(&self, endpoint: &str) -> Result<String>;
}
