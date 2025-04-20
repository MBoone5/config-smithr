use clap::Parser;
use git2::Repository;
use std::{
    fs::File,
    path::{Path, PathBuf},
    result::Result,
};

// Struct to encapsulate cfg/dot file info
#[derive(Parser)]
struct CliArgs {
    path: PathBuf,
}

struct ConfigFile {
    file: File,
}

struct ConfigSet {
    file_set: Vec<ConfigFile>,
}

struct Settings {
    repo: Repository,
}

struct SmithrState {
    file_set: ConfigSet,
    user_settings: Settings,
}

fn _get_repo_name(path: &Path) -> Result<String, String> {
    let repo_name = path
        .file_name()
        .and_then(|repo_os_str| repo_os_str.to_str());

    let result = match repo_name {
        None => {
            return Err(
                "Repo does not have a path on filesytem -- check your args passed when calling"
                    .to_string(),
            );
        }
        Some(name) => name.to_string(),
    };

    Ok(result)
}

fn validate_repo(repo: &Repository) -> bool {
    return repo.path().is_dir() && repo.is_bare();
}

fn get_repo(path: PathBuf) -> Result<Repository, String> {
    let repo = Repository::open(path);

    let result = match repo {
        Ok(repo) => {
            if validate_repo(&repo) {
                Ok(repo)
            } else {
                Err("Unable to open repo".to_string())
            }
        }
        Err(_) => Err("Unable to open repo".to_string()),
    };

    return result;
}

fn main() {
    let args = CliArgs::parse();

    println!("Reading repo at {:?} ... ", args.path);

    let repo = get_repo(args.path); // Its okay if we panic here

    match repo {
        Ok(..) => println!("Valid repo!"),
        Err(..) => panic!("Failed to get and validate repository"),
    }
}
