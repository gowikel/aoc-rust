//! Utility functions to read data from the file system

use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

/// Trait to access a file in the FileSystem
pub trait FileOpener {
    fn open_file<P>(&self, path: P) -> Result<impl Read>
    where
        P: AsRef<Path>;
}

/// Implementation of FileSystem that uses the local filesystem to fetch
/// files
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LocalFileSystem;

impl FileOpener for LocalFileSystem {
    fn open_file<P>(&self, path: P) -> Result<impl Read>
    where
        P: AsRef<Path>,
    {
        File::open(path)
    }
}

/// Returns the default implementation of FileSystem, which happens to be
/// a LocalFileSystem
pub fn get_default_fs_implementation() -> impl FileOpener {
    LocalFileSystem::default()
}
