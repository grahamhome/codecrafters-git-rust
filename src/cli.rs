use clap::{Parser, Subcommand};
use std::path::{PathBuf};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Subcommand)]
pub enum SubCommands {
    Init,
    CatFile {
        #[arg(short)]
        pretty_print: bool,
        hash: String,
    },
    HashObject {
        #[arg(short)]
        write: bool,
        file: PathBuf,
    },
    LsTree {
        #[arg(long)]
        name_only: bool,
        hash: String,
    },
    WriteTree,
}
