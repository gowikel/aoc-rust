use std::path::PathBuf;

pub fn get_data_path(input: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    PathBuf::from(manifest_dir)
        .join("tests")
        .join("test_data")
        .join(input)
}
