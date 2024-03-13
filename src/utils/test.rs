use std::env;
use std::fs;
use std::path::PathBuf;

/// Loads a fixture file from fixtures/{relative_path} and returns its contents as a string
pub fn load_fixture_as_string(relative_path: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("fixtures");
    path.push(relative_path);
    fs::read_to_string(path).expect("Failed to read fixture file")
}