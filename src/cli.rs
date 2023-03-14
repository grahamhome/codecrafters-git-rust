use clap::{Parser, Subcommand};
use std::path::{PathBuf};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,
}

/// The commands supported by this Git clone.
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
    CommitTree {
        hash: String,

        #[arg(short)]
        parent_hash: String,

        #[arg(short)]
        message: String,
    }
}
