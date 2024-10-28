pub mod cli;
pub mod constants;
pub mod fs;
pub mod http;
pub mod providers;
pub mod solvers;

use clap::ValueEnum;
use derive_more::Display;
use log::trace;
use std::fmt::Debug;
use thiserror::Error;

/// This identifies any AoC puzzle unequivocally
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

/// Which solution should the solver solve?
#[derive(Copy, Clone, PartialEq, Eq, Debug, Display, ValueEnum)]
pub enum Execute {
    ALL,
    P1,
    P2,
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

    /// Returns the stored year in the puzzle
    pub fn year(&self) -> u32 {
        self.year
    }

    /// Returns the stored day in the puzzle
    pub fn day(&self) -> u32 {
        self.day
    }
}
