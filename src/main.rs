// use duct::cmd;

use log::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

use clap::Parser;
use std::path::PathBuf;

use crate::bazel_data::{
    BazelData, fetch_bazel_data, fetch_images_targets_from_git_list, file_path_to_bazel_path,
};
use crate::git_data::{GitFileList, fetch_tracking_changed_git_list};

mod bazel_data;
mod git_data;
mod utils;

/// A simple CLI tool example using Clap
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional named argument: Name of the user
    #[arg(short, long)]
    targets_pat: Option<String>,
    // /// Named argument with a default value
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
    //
    // /// Flag argument: True if passed, false otherwise
    // #[arg(short, long)]
    // activate: bool,
}

fn main() {
    // Build and initialize a subscriber filtered strictly to DEBUG and above
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let git_list = fetch_tracking_changed_git_list();
    debug!("git_list = {git_list:?}");

    let targets_pat: Option<&str> = args.targets_pat.as_deref(); // .unwrap_or("default string");
    let files_targets = fetch_images_targets_from_git_list(&git_list, targets_pat);
    debug!("files_targets = {files_targets:?}");

    // let bazel_data = fetch_bazel_data();

    // let target_keys: Vec<String> = bazel_data.targets_files.keys().cloned().collect();
    for fpath in git_list.file_list {
        println!("changed file = {fpath}");
        if let Some(targets_vec) = files_targets.get(&fpath) {
            for target in targets_vec {
                println!("    => {target}");
            }
        }
    }
}
