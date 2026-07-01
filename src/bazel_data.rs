use std::{
    collections::{HashMap, HashSet},
    process::Command,
};

use tracing::debug;

use crate::{
    git_data::GitFileList,
    utils::{isdir, isfile, real_path, relative_path},
};

#[derive(Default, Debug)]
pub struct BazelData {
    pub targets_files: HashMap<String, Vec<String>>,
    pub files_targets: HashMap<String, Vec<String>>,
    pub workspace_dir: String,
}

fn extract_file_path_from_bazel_path(bazel_path: &String) -> Option<String> {
    // split string
    let tokens: Vec<&str> = bazel_path.split(" ").collect();
    for token in tokens {
        if !token.starts_with("//") {
            continue;
        }
        let skip2: String = token.chars().skip(2).collect();
        let maybe_file_path = skip2.replace(":", "/");
        if isfile(&maybe_file_path) {
            return Some(maybe_file_path);
        }
    }
    None
}

pub fn fetch_bazel_data() -> BazelData {
    let mut targets_files: HashMap<String, Vec<String>> = HashMap::new();
    let mut files_set: HashSet<String> = HashSet::new();
    let mut files_targets: HashMap<String, Vec<String>> = HashMap::new();

    // 1. get target list
    let mut targets: Vec<String> = vec![];
    let targets_output = Command::new("bazel")
        .arg("query")
        .arg("//...")
        .output()
        .expect("Failed to query targets");
    let targets_stdout = String::from_utf8_lossy(&targets_output.stdout).into_owned();

    for line in targets_stdout.lines() {
        if line.starts_with("//") {
            targets.push(line.to_string());
        }
    }

    for (ix, t) in targets.iter().enumerate() {
        debug!("info: targets[{ix}] = -->{t}<--");
    }

    // 2. make hashmap-of-lists of file paths per target
    for (_ix, target) in targets.iter().enumerate() {
        let last_arg = format!("kind('source file',deps({}))", target);
        let mut files_vec: Vec<String> = vec![];
        let files_output = Command::new("bazel")
            .arg("cquery")
            .arg(last_arg)
            .output()
            .expect("Failed to query targets");
        let files_stdout = String::from_utf8_lossy(&files_output.stdout).into_owned();
        for line in files_stdout.lines() {
            let trimmed_line = line.trim().to_string();
            debug!("re: {target} => evaluating file: {line}");
            if let Some(file_path) = extract_file_path_from_bazel_path(&trimmed_line) {
                files_vec.push(file_path.clone());
                files_set.insert(file_path.clone());
                debug!("re: {target} => confirmed file: {file_path}");
            }
        }
        targets_files.insert(target.clone(), files_vec);
    }

    // 3. make reverse hashmap-of-lists of targets per file path
    // let targets_vec: Vec<String> = targets_files.keys().cloned().collect();
    for file in files_set {
        let mut targets_vec: Vec<String> = vec![];
        for target in targets_files.keys() {
            if let Some(files_vec) = targets_files.get(target) {
                if files_vec.contains(&file) {
                    targets_vec.push(target.clone());
                }
            }
        }
        files_targets.insert(file, targets_vec);
    }

    // 4. fetch workspace directory
    let mut workspace_dir = String::from("");
    let workspace_output = Command::new("bazel")
        .arg("info")
        .arg("workspace")
        .output()
        .expect("Failed to query workspace");
    let workspace_stdout = String::from_utf8_lossy(&workspace_output.stdout).into_owned();
    for line in workspace_stdout.lines() {
        if isdir(&line.to_string()) {
            workspace_dir = format!("{}", line.trim());
        }
    }

    BazelData {
        targets_files,
        files_targets,
        workspace_dir,
    }
}

pub fn fetch_images_targets_from_git_list(
    gitlist: &GitFileList,
    targets_pat: Option<&str>,
) -> HashMap<String, Vec<String>> {
    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    let targets_pat = targets_pat.unwrap_or("");

    for gitpath in &gitlist.file_list {
        // let last_arg = format!("kind('.*', except(rdeps(//..., {gitpath}), {gitpath}))");

        // let last_arg = format!("kind('.*', rdeps(//my/..., //my:target) except //my:target)");
        // bazel query "rdeps(//my/..., //my:target) except //my:target"
        let last_arg = format!("kind('.*{targets_pat}.*', rdeps(//..., {gitpath}))");
        let tic = std::time::Instant::now();
        debug!("running query for {gitpath}");
        debug!("-->bazel cquery --keep_going {last_arg}<--");

        // bazel query "except(rdeps(//..., //path/to:target), //path/to:target)"

        // debug!("bazel query --keep_going
        let targets_output = Command::new("bazel")
            .arg("cquery")
            .arg("--keep_going")
            .arg(last_arg)
            .output()
            .expect("failed to query targets");
        debug!(" => took {:?}", tic.elapsed());
        let targets_stdout = String::from_utf8_lossy(&targets_output.stdout).into_owned();
        debug!(" => targets_stdout = {targets_stdout}");
        let mut targets_vec: Vec<String> = vec![];
        for line in targets_stdout.lines() {
            let trimmed_line = line.trim().to_string();
            // debug!("re: {target} => evaluating file: {line}");
            if trimmed_line.starts_with("//") {
                if trimmed_line.contains("null") {
                    continue;
                }
                for token in trimmed_line.split(" ") {
                    debug!("for {gitpath}, found target => {token}");
                    // if let Some(path_check) = extract_file_path_from_bazel_path(&token.to_string())
                    // {
                    //     debug!("path_check = {path_check}");
                    //     if gitpath.eq(&path_check) {
                    //         continue;
                    //     }
                    // }
                    targets_vec.push(token.to_string());
                    break;
                }
            }
        }
        out.insert(gitpath.clone(), targets_vec);
    }

    out
}

pub fn file_path_to_bazel_path(p: &String, bd: &BazelData) -> String {
    debug!("workspace_dir = {}", &bd.workspace_dir);

    // 1. get real path
    let real = real_path(p);
    debug!("real = {real}");

    // 2. get relative to bd.workspace_dir
    let relative = relative_path(&real, &bd.workspace_dir);
    debug!("relative = {relative}");

    // 3. prepend with '//'
    let out = format!("//{relative}");
    debug!("out = {out}");

    // return
    out
}
