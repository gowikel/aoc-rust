pub mod cli;
pub mod constants;

use anyhow::{Context, Result};
use log::{debug, trace};
use reqwest::header::COOKIE;
use std::env;
use std::fmt::Debug;

/// This identifies any AoC puzzle unequivocally
pub struct Puzzle {
    year: u32,
    day: u32,
}

impl Puzzle {
    /// Creates a new Puzzle input
    /// It will fail if the year or the day are outside the valid ranges
    pub fn new(year: u32, day: u32) -> Result<Self> {
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

        Ok(Self { year, day })
    }

    /// Downloads the main puzzle input for this puzzle
    pub fn download(&self) -> Result<String> {
        trace!("Downloading puzzle year: {}, day: {}", self.year, self.day);

        let base_url = "https://adventofcode.com";
        let endpoint =
            format!("{base_url}/{}/day/{}/input", self.year, self.day);
        let cookie = env::var(constants::AOC_COOKIE)
            .with_context(|| format!("{} not set", constants::AOC_COOKIE))?;
        let client = reqwest::blocking::Client::default();

        debug!("endpoint: {}", endpoint);

        let response = client
            .get(&endpoint)
            .header(COOKIE, cookie)
            .send()
            .with_context(|| {
                format!(
                    "Unable to download puzzle for {}/{}",
                    self.year, self.day
                )
            })?;

        debug!("response: {:?}", response);
        trace!("parsing text and returning");

        Ok(response
            .text()
            .with_context(|| "Unable to parse response text")?)
    }
}

impl Debug for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Puzzle {{ year: {}, day: {} }}", self.year, self.day)
    }
}
