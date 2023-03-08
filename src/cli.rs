use clap::{Parser, Subcommand};

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
    }
}
