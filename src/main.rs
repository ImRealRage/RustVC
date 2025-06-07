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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::init().unwrap(),
    }
}
