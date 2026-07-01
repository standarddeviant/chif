use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{
    collections::{HashMap, HashSet},
    process::Command,
};

use tracing::debug;

use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GitFileList {
    pub file_list: Vec<String>,
}

fn main() {
    let my_string = "hello world";

    // 1. Initialize the default hasher
    let mut hasher = DefaultHasher::new();

    // 2. Feed the string into the hasher
    my_string.hash(&mut hasher);

    // 3. Finalize and get the u64 hash value
    let result = hasher.finish();

    println!("Hash: {}", result);
}

pub fn git_state() -> String {
    let git_hash = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("Failed to query git commit hash");
    let git_hash_stdout = String::from_utf8_lossy(&git_hash.stdout).into_owned();

    let git_diff_hash = Command::new("git")
        .arg("diff")
        .output()
        .expect("Failed to hash git diff");

    // 1. Initialize the default hasher
    let mut hasher = DefaultHasher::new();

    // 2. Feed the git diff bytes into the hasher
    git_diff_hash.stdout.hash(&mut hasher);

    // 3. Finalize and get the u64 hash value
    let git_diff_hash_u64 = hasher.finish();

    format!("{}_{:0X}", git_hash_stdout, git_diff_hash_u64)
}

pub fn fetch_tracking_changed_git_list() -> GitFileList {
    let mut tracking_changed_files_list: Vec<String> = vec![];

    // 1. get target list
    let git_list_output = Command::new("git")
        .arg("diff")
        .arg("--relative")
        .arg("--name-only")
        .arg("@{u}")
        .output()
        .expect("Failed to query targets");
    let git_list_stdout = String::from_utf8_lossy(&git_list_output.stdout).into_owned();

    // println!("Yo, git_list_stdout = {git_list_stdout}");

    for line in git_list_stdout.lines() {
        // check if file exists?
        tracking_changed_files_list.push(line.to_string());
    }

    GitFileList {
        file_list: tracking_changed_files_list,
    }
}
