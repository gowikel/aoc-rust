pub mod cli;
pub mod constants;
pub mod http;

use anyhow::{Result};
use log::{debug, trace};
use std::fmt::Debug;

/// This identifies any AoC puzzle unequivocally
pub struct Puzzle<T>
where
    T: http::HTTPProvider,
{
    year: u32,
    day: u32,
    http_provider: T,
}

impl<T> Puzzle<T>
where
    T: http::HTTPProvider,
{
    /// Creates a new Puzzle input
    /// It will fail if the year or the day are outside the valid ranges
    pub fn new(year: u32, day: u32, http_provider: T) -> Result<Self>
    where
        T: http::HTTPProvider,
    {
        trace!("Creating new puzzle with year {}, day {}", year, day);

        if !constants::VALID_YEARS.contains(&year) {
            anyhow::bail!(format!(
                "year should be in [{}-{}] range. Current: {year}.",
                constants::VALID_YEARS.start(),
                constants::VALID_YEARS.end()
            ));
        }

        if !constants::VALID_DAYS.contains(&day) {
            anyhow::bail!(format!(
                "day should be in [{}-{}] range. Current: {day}].",
                constants::VALID_DAYS.start(),
                constants::VALID_DAYS.end()
            ));
        }

        Ok(Self {
            year,
            day,
            http_provider,
        })
    }

    /// Downloads the main puzzle input for this puzzle
    pub fn download(&self) -> Result<String> {
        trace!("Downloading puzzle year: {}, day: {}", self.year, self.day);

        let base_url = "https://adventofcode.com";
        let endpoint =
            format!("{base_url}/{}/day/{}/input", self.year, self.day);

        debug!("endpoint: {}", endpoint);

        let response = self.http_provider.get(&endpoint)?;

        debug!("response: {:?}", response);
        trace!("parsing text and returning");

        Ok(response)
    }
}

impl<T> Debug for Puzzle<T>
where
    T: http::HTTPProvider,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Puzzle {{ year: {}, day: {} }}", self.year, self.day)
    }
}
