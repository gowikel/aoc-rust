pub mod actions;
pub mod cli;
pub mod constants;
pub mod providers;

use log::trace;
use std::fmt::Debug;
use thiserror::Error;

/// This identifies any AoC puzzle unequivocally
pub struct Puzzle {
    year: u32,
    day: u32,
}

/// Puzzle creation Errors
#[derive(Error, Debug, PartialEq, Clone)]
pub enum PuzzleError {
    #[error("Invalid year: {0}")]
    InvalidYear(String),

    #[error("Invalid day: {0}")]
    InvalidDay(String),
}

impl Puzzle {
    /// Creates a new Puzzle input
    /// It will fail if the year or the day are outside the valid ranges
    pub fn new(year: u32, day: u32) -> Result<Self, PuzzleError> {
        trace!("Creating new puzzle with year {}, day {}", year, day);

        if !constants::VALID_YEARS.contains(&year) {
            return Err(PuzzleError::InvalidYear(format!(
                "year should be in [{}-{}] range. Current: {year}.",
                constants::VALID_YEARS.start(),
                constants::VALID_YEARS.end()
            )));
        }

        if !constants::VALID_DAYS.contains(&day) {
            return Err(PuzzleError::InvalidDay(format!(
                "day should be in [{}-{}] range. Current: {day}].",
                constants::VALID_DAYS.start(),
                constants::VALID_DAYS.end()
            )));
        }

        Ok(Self { year, day })
    }
}

impl Debug for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Puzzle {{ year: {}, day: {} }}", self.year, self.day)
    }
}
