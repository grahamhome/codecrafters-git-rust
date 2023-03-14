extern crate core;

use anyhow::{anyhow, Result};
use clap::Parser;
use cli::Cli;
use std::fs;

mod cat_file;
mod hash_object;
mod cli;
mod ls_tree;
mod object;
mod write_tree;

/// Parse the command-line arguments and execute the given command.
fn main() -> Result<()> {
    let git_cli = Cli::parse();
    match git_cli.command {
        // Initialize a directory as a Git repo.
        cli::SubCommands::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("Initialized git directory")
        }
        // Print the contents of a file with the given hash.
        cli::SubCommands::CatFile { pretty_print, hash } => {
            if !pretty_print {
                return Err(anyhow!("The -p flag is required"));
            }
            cat_file::pretty_cat_file(hash)?;
        }
        // Hash a given file and store its contents in a Git object file under the hash.
        cli::SubCommands::HashObject { write, file } => {
            if !write {
                return Err(anyhow!("The -w flag is required"));
            }
            println!("{}", hash_object::hash_and_write_file(file)?);
        }
        // Print file names in the file tree with the given hash.
        cli::SubCommands::LsTree { name_only, hash } => {
            if !name_only {
                return Err(anyhow!("The --name-only flag is required"));
            }
            ls_tree::ls_tree(hash)?;
        }
        // Hash the current directory and write its file tree to a Git object file under the
        // directory's hash.
        cli::SubCommands::WriteTree => {
            let hash = write_tree::write_tree(".")?;
            println!("{}", hash);
        }
    }
    Ok(())
}
