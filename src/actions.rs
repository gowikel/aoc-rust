//! This module provides the implementation of the ActionService struct,
//! which relies on abstractions over HTTP and file system providers.
//!
//! The ActionService struct offers concrete "actions" (hence the name),
//! that come in handy. Currently, I only implemented a download puzzle
//! input and an extract template function.

use crate::{
    providers::file_system::FileSystem,
    providers::http::{HTTPError, HTTPProvider},
    Puzzle,
};
use log::{debug, trace};
use std::{
    io::{Result as IOResult, Write},
    path::PathBuf,
    process::exit,
};

/// ActionService represents the service that performs actions related to Advent of Code
/// puzzles. It is parameterized over two types, H and F, which are the HTTP provider and
/// file system provider abstractions, respectively.
pub struct ActionService<H, F>
where
    H: HTTPProvider,
    F: FileSystem,
{
    http_provider: H,
    fs_provider: F,
}

impl<H, F> ActionService<H, F>
where
    H: HTTPProvider,
    F: FileSystem,
{
    /// Creates a new instance of ActionService.
    ///
    /// # Inputs
    ///
    /// * http_provider: An instance of a type that implements HTTPProvider.
    /// * fs_provider: An instance of a type that implements FileSystem.
    ///
    /// # Returns
    ///
    ///  A new ActionService instance.
    pub fn new(http_provider: H, fs_provider: F) -> Self {
        Self {
            http_provider,
            fs_provider,
        }
    }

    /// Downloads the input for a given Advent of Code puzzle.
    ///
    /// # Inputs
    ///
    /// * puzzle: A reference to a Puzzle containing the year and day of the puzzle.
    ///
    /// # Returns
    ///
    /// A Result containing the puzzle input text if successful,
    /// or an HTTPError otherwise.
    pub fn download_input(&self, puzzle: &Puzzle) -> Result<String, HTTPError> {
        trace!(
            "Downloading puzzle year: {}, day: {}",
            puzzle.year,
            puzzle.day
        );

        let base_url = "https://adventofcode.com";
        let endpoint =
            format!("{base_url}/{}/day/{}/input", puzzle.year, puzzle.day);

        debug!("endpoint: {}", endpoint);

        let response = self.http_provider.get(&endpoint)?;

        debug!("response: {:?}", response);
        trace!("parsing text and returning");

        Ok(response)
    }

    /// Extracts a template and writes it to a file system path based on the puzzle's
    /// year and day.
    ///
    /// # Inputs
    ///
    /// * puzzle: A reference to a Puzzle containing the year and day of the puzzle.
    ///
    /// # Returns
    ///
    /// An IOResult indicating success or failure of the file operations.
    pub fn extract_template_for(&self, puzzle: &Puzzle) -> IOResult<()> {
        trace!("Extracting template...");
        let template = include_str!("../templates/day_template.txt")
            .replace("#DAY", format!("{:02}", puzzle.day()).as_str());

        let target: PathBuf = [
            ".",
            "src",
            "solvers",
            format!("y{}", puzzle.year()).as_str(),
            format!("day{:02}.rs", puzzle.day()).as_str(),
        ]
        .iter()
        .collect::<PathBuf>();

        if self.fs_provider.exists(&target) {
            eprintln!("{} already exists!", target.display());
            eprintln!("Please remove the file before trying again.");
            eprintln!("Or add the force option.");

            exit(exitcode::USAGE);
        }

        debug!("Creating and writing to {}...", target.display());
        let mut file = self.fs_provider.open(&target)?;
        write!(file, "{}", template)?;

        debug!("Wrote {} to buffer", target.display());
        debug!("Extraction finished!");

        Ok(())
    }

    /// Internally calls the [`set_cookie`] method of the [`HTTPProvider`]
    pub fn set_cookie(&mut self, cookie: String) {
        self.http_provider.set_cookie(cookie);
    }
}
