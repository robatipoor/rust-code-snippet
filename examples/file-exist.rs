use std::fs;
use std::path::Path;

fn file_exist(file: &str) -> bool {
    Path::new(file).exists()
}

fn main() {
    assert_eq!(true, file_exist("Cargo.toml"));
}
