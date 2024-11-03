use log::{debug, trace};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Result as IOResult, Write};
use std::path::PathBuf;

/// A trait for basic file system operations.
///
/// This trait defines the minimum set of methods required to check for the
/// existence of a file and to open a file for writing.
pub trait FileSystem {
    /// The type of writer used for writing to the file.
    type Writer: Write;

    /// Checks if the file at the given `path` exists.
    ///
    /// # Arguments
    ///
    /// * `path` - A `PathBuf` indicating the path of the file to check.
    ///
    /// # Returns
    ///
    /// * `true` if the file exists, `false` otherwise.
    fn exists(&self, path: &PathBuf) -> bool;

    /// Opens the file at the given `path` for writing.
    ///
    /// This method returns a [`IOResult`] containing a writer if the file is
    /// successfully opened, or an I/O error if it fails.
    ///
    /// # Arguments
    ///
    /// * `path` - A `PathBuf` indicating the path of the file to open.
    ///
    /// # Returns
    ///
    /// * An [`IOResult`] which is either:
    ///   - `Ok(Self::Writer)`: A writer for writing to the file.
    ///   - `Err(e)`: An I/O error if the file cannot be opened.
    fn open_writable(&self, path: &PathBuf) -> IOResult<Self::Writer>;
}

/// A local file system implementation of the `FileSystem` trait.
///
/// This struct provides functionality to check for file existence and to open files
/// using standard library types.
#[derive(Default)]
pub struct LocalFSAdapter;

impl FileSystem for LocalFSAdapter {
    type Writer = BufWriter<File>;

    fn exists(&self, path: &PathBuf) -> bool {
        path.exists()
    }

    fn open_writable(&self, path: &PathBuf) -> IOResult<Self::Writer> {
        trace!("open {}", path.display());
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        debug!("File opened...");
        let buf = BufWriter::new(file);
        Ok(buf)
    }
}
