use clap::Parser;
use git2::Repository;
use std::path::PathBuf;

// Struct to encapsulate cfg/dot file info
#[derive(Parser)]
struct Forge {
    path: PathBuf,
}

// TODO: Figure out how to unwrap `open_bare` into repo.namespace() to get repo name
fn check_repo(path: PathBuf) {
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let repo_name = repo.path()
        .file_name()
        .and_then(|repo_os_str| repo_os_str.to_str());

    println!("Bare repo? = {}", repo.is_bare());

    match repo_name {
        None => println!("Repo name not found, check filesystem"),
        Some(name) => println!("Repo name? = {}", name)
    }
}

fn main() {
    let args = Forge::parse();

    println!("Reading repo at {:?} ... ", args.path);
    check_repo(args.path);
}
