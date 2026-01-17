mod cli;
mod crypto;
mod vault;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode {
            files,
            out,
            threads,
        } => {
            vault::encode::run(files, out, threads);
        }
        Commands::Decode {
            vault,
            seed,
            out_dir,
            threads,
        } => {
            vault::decode::run(vault, seed, out_dir, threads);
        }
    }
}
