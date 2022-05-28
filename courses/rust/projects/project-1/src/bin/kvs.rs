use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"))]
#[clap(author = env!("CARGO_PKG_AUTHORS"))]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
#[clap(disable_help_subcommand(true))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Set key-value
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Set { key: _, value: _} => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Get { key: _ } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Rm { key: _ } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
