use std::{
    collections::{HashMap, HashSet},
    process::Command,
};

use tracing::debug;

#[derive(Default, Debug)]
pub struct GitFileList {
    pub file_list: Vec<String>,
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
