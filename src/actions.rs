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
    }

    impl HTTPProviderMock {
        fn success(data: &str) -> Self {
            Self {
                data: Ok(data.to_string()),
            }
        }

        fn failed(error: HTTPError) -> Self {
            Self { data: Err(error) }
        }
    }

    impl HTTPProvider for HTTPProviderMock {
        fn get(&self, _endpoint: &str) -> Result<String, HTTPError> {
            self.data.clone()
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
    fn test_missing_aoc_env_var() {
        let puzzle = Puzzle { year: 2023, day: 1 };
        let deps = HTTPProviderMock::failed(HTTPError::MissingEnvVarError(
            std::env::VarError::NotPresent,
        ));
        let result = download_input(&deps, puzzle);

        assert_eq!(
            result,
            Err(HTTPError::MissingEnvVarError(
                std::env::VarError::NotPresent
            ))
        );
    }
}
