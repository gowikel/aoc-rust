pub mod cli;

use anyhow::{Context, Result};
use log::{debug, trace};
use reqwest::header::COOKIE;
use std::env;
use std::fmt::Debug;
use std::ops::RangeInclusive;

/// This identifies any AoC puzzle unequivocally
pub struct Puzzle {
    year: u32,
    day: u32,
}

const VALID_YEARS: RangeInclusive<u32> = 2023..=2023;
const VALID_DAYS: RangeInclusive<u32> = 1..=25;

impl Puzzle {
    /// Creates a new Puzzle input
    /// It will fail if the year or the day are outside the valid ranges
    pub fn new(year: u32, day: u32) -> Result<Self> {
        trace!("Creating new puzzle with year {}, day {}", year, day);

        if !VALID_YEARS.contains(&year) {
            anyhow::bail!(format!(
                "year should be in [{}-{}] range. Current: {year}.",
                VALID_YEARS.start(),
                VALID_YEARS.end()
            ));
        }

        if !VALID_DAYS.contains(&day) {
            anyhow::bail!(format!(
                "day should be in [{}-{}] range. Current: {day}].",
                VALID_DAYS.start(),
                VALID_DAYS.end()
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
        let cookie =
            env::var("AOC_COOKIE").with_context(|| "AOC_COOKIE not set")?;
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
