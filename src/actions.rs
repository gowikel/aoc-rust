use crate::{
    providers::http::{HTTPError, HTTPProvider},
    Puzzle,
};
use log::{debug, trace};

/// Downloads the main puzzle input for this puzzle
pub fn download_input<D>(deps: &D, puzzle: Puzzle) -> Result<String, HTTPError>
where
    D: HTTPProvider,
{
    trace!(
        "Downloading puzzle year: {}, day: {}",
        puzzle.year,
        puzzle.day
    );

    let base_url = "https://adventofcode.com";
    let endpoint =
        format!("{base_url}/{}/day/{}/input", puzzle.year, puzzle.day);

    debug!("endpoint: {}", endpoint);

    let response = deps.get(&endpoint)?;

    debug!("response: {:?}", response);
    trace!("parsing text and returning");

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::http::{HTTPError, HTTPProvider};

    struct HTTPProviderMock {
        data: Result<String, HTTPError>,
        cookie: Option<String>,
    }

    impl HTTPProviderMock {
        fn success(data: &str) -> Self {
            Self {
                data: Ok(data.to_string()),
                cookie: None,
            }
        }

        fn failed(error: HTTPError) -> Self {
            Self {
                data: Err(error),
                cookie: None,
            }
        }
    }

    impl HTTPProvider for HTTPProviderMock {
        fn get(&self, _endpoint: &str) -> Result<String, HTTPError> {
            self.data.clone()
        }

        fn set_cookie(&mut self, cookie: String) {
            self.cookie = Some(cookie);
        }

        fn get_cookie(&self) -> Option<String> {
            self.cookie.clone()
        }
    }

    #[test]
    fn test_happy_path() {
        let puzzle = Puzzle { year: 2023, day: 1 };
        let deps = HTTPProviderMock::success("PUZZLE-DATA");
        let result = download_input(&deps, puzzle);

        assert_eq!(result, Ok("PUZZLE-DATA".to_string()));
    }

    #[test]
    fn test_fetch_error() {
        let puzzle = Puzzle { year: 2023, day: 1 };
        let deps = HTTPProviderMock::failed(HTTPError::FetchError(
            "Unable to fetch data".into(),
        ));
        let result = download_input(&deps, puzzle);

        assert_eq!(
            result,
            Err(HTTPError::FetchError("Unable to fetch data".into()))
        );
    }
}
