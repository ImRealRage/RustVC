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
    /// Initialize a new repository
    Init,
    /// Compute and display the hash of a file
    HashObject {
        /// Path of the file to hash
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::init().unwrap(),
        Commands::HashObject { file } => commands::hash_object::hash_object(&file).unwrap(),
    }
}
