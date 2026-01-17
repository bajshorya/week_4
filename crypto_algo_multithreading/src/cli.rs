use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        threads: usize,
    },
    Decode {
        #[arg(long)]
        vault: PathBuf,
        #[arg(long)]
        seed: String,
        #[arg(long)]
        out_dir: PathBuf,
        #[arg(long)]
        threads: usize,
    },
}
