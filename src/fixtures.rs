// compile this module only when building tests
#![cfg(test)]

use std;

pub struct Fixtures {}

impl Fixtures {
    pub fn git_dir() -> std::path::PathBuf {
        let cwd = std::env::current_dir().unwrap();
        cwd.join("tests").join("fixtures").join("dotgit")
    }

    pub fn git_commit_sha() -> String {
        String::from("3a5c191cc2a02f55de5bd3478ff5ff9038a32dad")
    }

    // pub fn git_tree_sha() -> String {
    //     String::from("3fb6f8e88d177050ec4eb322da24c60a82fde341")
    // }
    //
    // pub fn git_blob_sha() -> String {
    //     String::from("452bfc78736ed6616e4402931ba587d6172ca027")
    // }
}
