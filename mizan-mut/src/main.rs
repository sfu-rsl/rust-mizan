use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod mutate;
mod mutations;
mod rename;

use mutate::Mutation;

#[derive(Parser, Debug)]
#[command(name = "mizan-mut")]
#[command(about = "Rust code mutation and refactoring tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Apply semantic-preserving mutations to Rust source code
    Mutate {
        #[arg(short, long, help = "Root directory of the crate to mutate")]
        root: PathBuf,
        #[arg(
            short,
            long,
            value_enum,
            help = "Mutations to apply (can be specified multiple times)"
        )]
        mutations: Vec<Mutation>,
    },
    /// Rename any symbol in Rust codebases using rust-analyzer
    Rename {
        /// Path to the crate root (directory containing Cargo.toml)
        #[arg(short, long)]
        crate_root: String,

        /// Path to the file containing the symbol (relative to crate root)
        #[arg(short, long)]
        file: String,

        /// Byte offset of the symbol in the file
        #[arg(short, long)]
        offset: u32,

        /// New name for the symbol
        #[arg(short, long)]
        new_name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Mutate { root, mutations } => {
            mutate::apply_mutations(&root, mutations)?;
        }
        Commands::Rename {
            crate_root,
            file,
            offset,
            new_name,
        } => {
            rename::rename_symbol(&crate_root, &file, offset, &new_name)?;
        }
    }

    Ok(())
}
