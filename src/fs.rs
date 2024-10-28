use crate::providers::file_system::{FileSystem, LocalFileSystem};
use crate::Puzzle;
use log::{debug, trace};
use std::io::{Result as IOResult, Write};
use std::path::PathBuf;
use std::process::exit;

pub struct FSService<F>
where
    F: FileSystem,
{
    fs_provider: F,
}

impl<F> FSService<F>
where
    F: FileSystem,
{
    pub fn new(fs_provider: F) -> Self {
        Self { fs_provider }
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
}

impl Default for FSService<LocalFileSystem> {
    /// Returns a default implementation of FSService
    fn default() -> Self {
        Self::new(LocalFileSystem)
    }
}
