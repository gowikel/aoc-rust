use crate::providers::http::{HTTPAdapter, HTTPError, HTTPProvider};
use crate::Puzzle;
use log::{debug, trace};

pub struct HTTPService<H>
where
    H: HTTPProvider,
{
    http_provider: H,
}

impl<H> HTTPService<H>
where
    H: HTTPProvider,
{
    /// Creates a new instance of HTTPService.
    ///
    /// # Inputs
    ///
    /// * http_provider: An instance of a type that implements HTTPProvider.
    ///
    /// # Returns
    ///
    ///  A new HTTPService instance.
    pub fn new(http_provider: H) -> Self {
        Self { http_provider }
    }

    /// Downloads the input for a given Advent of Code puzzle.
    ///
    /// # Inputs
    ///
    /// * puzzle: A reference to a Puzzle containing the year and day of the puzzle.
    ///
    /// # Returns
    ///
    /// A Result containing the puzzle input text if successful,
    /// or an HTTPError otherwise.
    pub fn download_input(&self, puzzle: &Puzzle) -> Result<String, HTTPError> {
        trace!(
            "Downloading puzzle year: {}, day: {}",
            puzzle.year,
            puzzle.day
        );

        let base_url = "https://adventofcode.com";
        let endpoint =
            format!("{base_url}/{}/day/{}/input", puzzle.year, puzzle.day);

        debug!("endpoint: {}", endpoint);

        let response = self.http_provider.get(&endpoint)?;

        debug!("response: {:?}", response);
        trace!("parsing text and returning");

        Ok(response)
    }

    /// Internally calls the [`set_cookie`] method of the [`HTTPProvider`]
    pub fn set_cookie(&mut self, cookie: String) {
        self.http_provider.set_cookie(cookie);
    }
}

impl Default for HTTPService<HTTPAdapter> {
    /// Returns an HTTPService already configured with [`HTTPAdapter`]
    fn default() -> Self {
        Self::new(HTTPAdapter::default())
    }
}
