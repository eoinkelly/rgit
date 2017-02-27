#[macro_use]
extern crate lazy_static;

extern crate compress;
extern crate regex;

mod git_sha;
mod git_object;
mod git_object_finder;

// only declare the fixtures module if we are building for tests
#[cfg(test)]
mod fixtures;

const MISSING_SHA_ERROR: &'static str = "You must supply a SHA1 hash";

fn main() {
    let sha = git_sha::GitSha::from(read_sha_from_cmdline());

    let cwd = std::env::current_dir().unwrap();
    let gitdir_path = cwd.join(".git");
    println!("Searching git repository in: {:?}", gitdir_path);

    let git_object = git_object_finder::GitObjectFinder::find(gitdir_path, sha);
    println!("GitObject: {}", git_object);
}

fn read_sha_from_cmdline() -> String {
    // args[0] = name of program
    // args[1] = SHA1 checksum
    match std::env::args().nth(1) {
        Some(sha) => sha,
        None => exit_with_error(MISSING_SHA_ERROR),
    }
}

fn exit_with_error(msg: &str) -> ! {
    println!("Error: {}", msg);
    std::process::exit(-1);
}
