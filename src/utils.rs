use std::fs;
use std::path::{Path, PathBuf};

pub fn isfile(x: &String) -> bool {
    let path = Path::new(x);

    // Check if the path points to a file
    let is_file = fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false);

    is_file
}

pub fn isdir(x: &String) -> bool {
    let path = Path::new(x);

    // Check if the path points to a file
    let is_dir = fs::metadata(path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false);

    is_dir
}

pub fn real_path(x: &String) -> String {
    let path = PathBuf::from(x);
    if let Ok(real_path) = fs::canonicalize(&path) {
        return format!("{}", real_path.to_string_lossy());
    }

    return String::from(""); // return empty string on fail paths
}

// 'full_path relative to base'
pub fn relative_path(full_path_in: &String, base_in: &String) -> String {
    let base = Path::new(base_in);
    let full_path = Path::new(full_path_in);

    if let Ok(relative_path) = full_path.strip_prefix(base) {
        return format!("{}", relative_path.to_string_lossy());
    }

    String::from("")
}
