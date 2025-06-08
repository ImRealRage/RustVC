mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustvc")]
#[command(about = "A minimal version control system in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    HashObject {
        file: String,
    },
    /// Show content of object by hash
    CatFile {
        /// Hash of the object to display
        hash: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::init().unwrap(),
        Commands::HashObject { file } => commands::hash_object::hash_object(&file).unwrap(),
        Commands::CatFile { hash } => commands::cat_file::cat_file(&hash).unwrap(),
    }
}
