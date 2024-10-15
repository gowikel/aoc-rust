pub mod actions;
pub mod cli;
pub mod constants;
pub mod http;

use anyhow::Result;
use log::trace;
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
}

impl Debug for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Puzzle {{ year: {}, day: {} }}", self.year, self.day)
    }
}
