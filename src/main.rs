use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
enum Commands {
    Tokenize {
        filename: PathBuf
    },
    Parse {
        filename: PathBuf
    },
    Eval {
        filename: PathBuf
    },
    Run {
        filename: PathBuf
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

fn main() -> miette::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Tokenize { filename } => todo!(),
        Commands::Parse { filename } => todo!(),
        Commands::Eval { filename } => todo!(),
        Commands::Run { filename } => todo!(),
    }

    Ok(())
}
