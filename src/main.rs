// use duct::cmd;

use log::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

use crate::bazel_data::{BazelData, fetch_bazel_data, file_path_to_bazel_path};
use crate::git_data::{GitFileList, fetch_tracking_changed_git_list};

mod bazel_data;
mod git_data;
mod utils;

fn main() {
    // Build and initialize a subscriber filtered strictly to DEBUG and above
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let bazel_data = fetch_bazel_data();
    let git_list = fetch_tracking_changed_git_list();

    let target_keys: Vec<String> = bazel_data.targets_files.keys().cloned().collect();
    for fpath in git_list.file_list {
        // let pos_target_list: Vec<String> = vec![];

        println!("changed file = {fpath}");

        // NOTE: 'bpath' means 'bazel path', useful to search among bazel paths
        // let bpath = file_path_to_bazel_path(&fpath, &bazel_data);
        for target in &target_keys {
            if let Some(file_list) = bazel_data.targets_files.get(target) {
                for fcheck in file_list {
                    debug!("checking \n    git: -->{fpath}<--\n    bazel: -->{fcheck}<--");
                }

                // TODO: make file check more robust than simple string equality
                if file_list.contains(&fpath) {
                    println!("  => used by target {target}");
                }
            }
        }
    }
}
