use super::http::{get_http_provider};
use super::Puzzle;
use anyhow::Result;
use log::{debug, trace};

/// Downloads the main puzzle input for this puzzle
pub fn download_input(puzzle: Puzzle) -> Result<String> {
    trace!(
        "Downloading puzzle year: {}, day: {}",
        puzzle.year,
        puzzle.day
    );

    let http_provider =
        get_http_provider().expect("Failed to get HTTP provider");

    let base_url = "https://adventofcode.com";
    let endpoint =
        format!("{base_url}/{}/day/{}/input", puzzle.year, puzzle.day);

    debug!("endpoint: {}", endpoint);
    
    let response = http_provider.get(&endpoint)?;

    debug!("response: {:?}", response);
    trace!("parsing text and returning");

    Ok(response)
}
