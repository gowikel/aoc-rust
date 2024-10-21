use crate::{
    providers::data::FileOpener,
    providers::http::{HTTPError, HTTPProvider},
    Puzzle,
};
use log::{debug, trace};
use std::{
    io::{BufReader, Read, Result as IOResult},
    path::Path,
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

/// Loads the specified file input and returning a BufferReader
pub fn open_file_buffer<'a>(
    deps: &'a impl FileOpener,
    path: &'a Path,
) -> IOResult<BufReader<impl Read + 'a>> {
    let file = deps.open_file(path)?;
    Ok(BufReader::new(file))
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

    mod open_file_buffer {
        use super::super::*;
        use crate::providers::data::FileOpener;
        use std::io::{Cursor, Read};
        use std::path::{Path, PathBuf};

        struct FileBufferMock {
            data: Vec<u8>,
        }

        impl FileBufferMock {
            fn new(data: &str) -> Self {
                let bytes = data.as_bytes().to_vec();

                Self { data: bytes }
            }
        }

        impl FileOpener for FileBufferMock {
            fn open_file<P: AsRef<Path>>(
                &self,
                _path: P,
            ) -> IOResult<impl Read> {
                let cursor = Cursor::new(self.data.clone());
                Ok(cursor)
            }
        }

        #[test]
        fn file_buffer_with_data() {
            let mock = FileBufferMock::new("DATA");
            let path: PathBuf = ["does", "not", "matter"].iter().collect();

            let buffer = open_file_buffer(&mock, &path);

            assert!(buffer.is_ok());
            let mut result: Vec<u8> = Vec::new();
            buffer.unwrap().read_to_end(&mut result).unwrap();
            let result = String::from_utf8(result).unwrap();

            assert_eq!(result, "DATA");
        }

        #[test]
        fn empty_buffer() {
            let mock = FileBufferMock::new("");
            let path: PathBuf = ["does", "not", "matter"].iter().collect();

            let buffer = open_file_buffer(&mock, &path);

            assert!(buffer.is_ok());

            let mut result: Vec<u8> = Vec::new();
            buffer.unwrap().read_to_end(&mut result).unwrap();
            let result = String::from_utf8(result).unwrap();

            assert_eq!(result, "");
        }
    }
}
