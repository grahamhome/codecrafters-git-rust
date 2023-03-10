extern crate core;

use anyhow::{anyhow, Result};
use clap::Parser;
use cli::Cli;
use std::fs;

mod cat_file;
mod hash_object;
mod cli;

fn main() -> Result<()> {
    let git_cli = Cli::parse();
    match git_cli.command {
        cli::SubCommands::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("Initialized git directory")
        }
        cli::SubCommands::CatFile { pretty_print, hash } => {
            cat_file::pretty_cat_file(hash)?;
        }
        cli::SubCommands::HashObject { write, file } => {
            println!("{}", hash_object::hash_and_write_file(file)?);
        }
    }
    Ok(())
}
