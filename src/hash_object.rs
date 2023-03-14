use crate::object::{store_object, GitObject};
use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Given a path to a file, reads in the content of the file, creates a
/// GitObject instance with type "blob" and the file contents, and
/// calls store_object to create a hash from the file's header + content
/// and use this hash to create a file containing the input file's ZLib-encoded
/// header + contents.
pub fn hash_and_write_file<P: AsRef<Path>>(path: P) -> Result<String> {
    // Open source file and read contents into buffer
    let source_file = File::open(path)?;
    let mut reader = BufReader::new(source_file);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // Create GitObject and write it to a file.
    store_object(GitObject {
        object_type: "blob".to_string(),
        content: buffer,
    })
}