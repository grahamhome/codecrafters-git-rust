use crate::hash_object::hash_and_write_file;
use crate::object::{store_object, GitObject};
use anyhow::Result;
use std::fs::{read_dir, DirEntry};
use std::os::unix::ffi::OsStrExt;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

const DIRECTORY_FLAG: u16 = 1 << 14;  // 0100
const FILE_FLAG: u16 = 1 << 15;  // 1000

/// Given a path to a directory, traverses the directory's files and subfolders.
/// Writes each subfolder's file tree to a Git object file.
/// Writes all files' contents to their own Git object files.
pub fn write_tree<P: AsRef<Path>>(path:P) -> Result<String> {
    // Read files in directory and sort alphabetically
    let mut files: Vec<DirEntry> = read_dir(path)?.map(|f| f.unwrap()).collect();
    files.sort_by_key(|f| f.file_name());

    let mut content: Vec<u8> = Vec::new();

    // Add all file names to file tree contents
    // DirEntry type represents both files and directories.
    for file in files {
        // Ignore directory names starting with a period
        if file.file_name().to_str().unwrap().starts_with('.') {
            continue;
        }

        // Get hash for each file or folder in directory
        let file_type = file.file_type()?;
        let (hash, mode) = if file_type.is_dir() {
            // Recurse into directory and write its file trees and files to Git object files.
            // Get the hash of the directory's file tree.
            (write_tree(file.path())?, DIRECTORY_FLAG)
        } else if file_type.is_file() {
            // Write file header + contents to a Git object file. Get file hash.
            let hash = hash_and_write_file(file.path())?;

            #[cfg(unix)]
                let file_mode = file.metadata()?.permissions().mode();
            #[cfg(not(unix))]
                let file_mode = 0o644;

            // File mode is file's permissions | file_flag
            (hash, FILE_FLAG | ((file_mode & 0o777) as u16))
        } else {
            // Symlinks not supported
            continue;
        };

        // Add the mode, name, and hash of the file or subdirectory to this file tree's contents
        // Hash is decoded, is this because we are going to re-encode it?
        // This must be why each hash in the file tree is 20 chars not 40: 2 chars in a hex
        // string = 1 ASCII char.
        content.extend_from_slice(format!("{:o} ", mode).as_bytes());
        content.extend_from_slice(file.file_name().as_bytes());
        content.push(0);
        content.append(&mut hex::decode(hash)?);
    }

    // Write file tree contents to a Git object file (re-encoding and hashing them in the process)
    // Get file tree hash and return it.
    let hash = store_object(GitObject {
        object_type: "tree".to_string(),
        content,
    })?;
    Ok(hash)
}