use log::{debug, trace};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Result as IOResult, Write};
use std::path::PathBuf;

/// This trait defines files that can be opened/created in write mode
pub trait FSWrite {
    /// The type of writer used for writing to the file.
    type Writer: Write;

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
    fn open(&self, path: &PathBuf) -> IOResult<Self::Writer>;
}

/// This trait defines an operation to check if a file indeed exists in the filesystem
pub trait FSExists {
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
}

/// A local file system implementation of the `FileSystem` trait.
///
/// This struct provides functionality to check for file existence and to open files
/// using standard library types.
#[derive(Default)]
pub struct LocalFSAdapter;

impl FSWrite for LocalFSAdapter {
    type Writer = BufWriter<File>;

    fn open(&self, path: &PathBuf) -> IOResult<Self::Writer> {
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

impl FSExists for LocalFSAdapter {
    fn exists(&self, path: &PathBuf) -> bool {
        path.exists()
    }
}
