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

#[cfg(test)]
mod tests {
    use crate::providers::http::tests::HttpProviderMock;
    mod download_input {
        use super::super::*;
        use super::*;
        use crate::providers::http::HTTPError;
        use crate::Puzzle;

        #[test]
        fn happy_path() {
            let puzzle = Puzzle::new(2023, 5).unwrap();
            let expected_endpoint = "https://adventofcode.com/2023/day/5/input";

            let mut mock = HttpProviderMock::new();
            mock.insert_response(
                expected_endpoint.to_string(),
                "Mocked response data".to_string(),
            );
            mock.assert_called_with(expected_endpoint.to_string());

            let service = HTTPService::new(mock);
            let result = service.download_input(&puzzle);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Mocked response data");
        }

        #[test]
        fn error_propagation() {
            let puzzle = Puzzle::new(2023, 5).unwrap();
            let endpoint = "https://adventofcode.com/2023/day/5/input";

            let mut mock = HttpProviderMock::new();
            mock.insert_error(
                endpoint.to_string(),
                HTTPError::FetchError("Mocked fetch error".to_string()),
            );

            let service = HTTPService::new(mock);
            let result = service.download_input(&puzzle);

            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                HTTPError::FetchError("Mocked fetch error".to_string())
            );
        }
    }

    mod set_cookie {
        use super::super::*;
        use super::*;

        #[test]
        fn test_cookie_is_set() {
            let cookie = "TEST-COOKIE";
            let mut mock = HttpProviderMock::new();

            mock.assert_cookie_value(cookie.to_owned());

            let mut service = HTTPService::new(mock);
            service.set_cookie(cookie.to_string());
        }
    }
}
