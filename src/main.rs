use clap::Parser;
use std::path::PathBuf;

// Struct to encapsulate cfg/dot file info
#[derive(Parser)]
struct Forge {
    pattern: String,
    path: PathBuf,
}

fn main() {
    let args = Forge::parse();
    println!("pattern: {:?}", args.pattern);
}
