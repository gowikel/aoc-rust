use aoc::providers::data::{FileOpener, LocalFileSystem};
use std::io::{Read, Write};
use tempfile::NamedTempFile;

#[test]
fn test_read_file() {
    let mut file = NamedTempFile::new().unwrap();
    let data = "test data";
    write!(file, "{}", data).unwrap();

    let file_path = file.path();
    let local_fs = LocalFileSystem::default();
    let buffer = local_fs.open_file(file_path);

    assert!(buffer.is_ok());

    let mut buffer = buffer.unwrap();
    let mut result = String::new();
    buffer.read_to_string(&mut result).unwrap();
    assert_eq!(result, data);
}
