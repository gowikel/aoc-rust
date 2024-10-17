//! Dependencies to make HTTP requests

use anyhow::Result as AnyhowResult;
use log::trace;
use std::sync::{Arc, OnceLock};

/// Trait to build and send HTTP Requests (not async)
pub trait HTTPProvider {
    /// Prepares a GET request to the specified endpoint
    fn get(&self, endpoint: &str) -> AnyhowResult<String>;
}

static HTTP_PROVIDER: OnceLock<Arc<dyn HTTPProvider + Send + Sync>> =
    OnceLock::new();

pub fn init<P>(provider: P) -> Result<(), &'static str>
where
    P: HTTPProvider + Send + Sync + 'static,
{
    trace!("Initializing HTTP provider...");
    HTTP_PROVIDER
        .set(Arc::new(provider))
        .map_err(|_| "Unable to initialize the HTTP Provider")?;

    Ok(())
}

pub fn get_http_provider() -> Option<Arc<dyn HTTPProvider + Send + Sync>> {
    HTTP_PROVIDER.get().cloned()
}
