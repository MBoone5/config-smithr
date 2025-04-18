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
    let repo_result = Repository::open_bare(path);
    let repo = match repo_result {
        Ok(repo) => repo,
        Err(error) => {
            panic!("nah nah nah nah: {}", error);
        }
    };

    println!("{}", repo.is_bare());
}

fn main() {
    let args = Forge::parse();

    println!("Reading repo at {:?} ... ", args.path);
    check_repo(args.path);
}
