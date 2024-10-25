use crate::{
    providers::http::{HTTPError, HTTPProvider},
    Puzzle,
};
use log::{debug, trace};
use std::{
    fs::File,
    io::{BufWriter, Result as IOResult, Write},
    path::PathBuf,
    process::exit,
};

/// Downloads the main puzzle input for this puzzle
pub fn download_input(
    deps: &impl HTTPProvider,
    puzzle: Puzzle,
) -> Result<String, HTTPError> {
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

/// Copies the template to the given path
/// # Panics
/// This function will panic if:
/// - The input file already exists OR
/// - The module definition file (yYYYY.rs) does not exist
pub fn extract_template_for(puzzle: &Puzzle) -> IOResult<()> {
    trace!("Extracting template...");
    let template = include_str!("../templates/day_template.txt")
        .replace("#DAY", puzzle.day().to_string().as_str());

    let target: PathBuf = [
        ".",
        "src",
        "solvers",
        format!("y{}", puzzle.year()).as_str(),
        format!("day{:02}.rs", puzzle.day()).as_str(),
    ]
    .iter()
    .collect::<PathBuf>();

    if target.exists() {
        eprintln!("{} already exists!", target.display());
        eprintln!("Please remove the file before trying again.");
        eprintln!("Or add the force option.");

        exit(exitcode::USAGE);
    }

    debug!("Creating and writing to {}...", target.display());
    let file = File::create(target.clone())?;
    let mut buffer = BufWriter::new(file);
    write!(buffer, "{}", template)?;

    debug!("Wrote {} to buffer", target.display());
    debug!("Extraction finished!");

    Ok(())
}

#[cfg(test)]
mod tests {
    mod download_input {
        use super::super::*;
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
}
