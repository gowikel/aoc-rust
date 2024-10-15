use super::http::HTTPProvider;
use super::Puzzle;
use anyhow::Result;
use log::{debug, trace};

/// Downloads the main puzzle input for this puzzle
pub fn download_input<T>(http_provider: T, puzzle: Puzzle) -> Result<String>
where
    T: HTTPProvider,
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

    let response = http_provider.get(&endpoint)?;

    debug!("response: {:?}", response);
    trace!("parsing text and returning");

    Ok(response)
}
