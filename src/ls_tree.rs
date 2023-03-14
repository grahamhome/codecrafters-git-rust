use crate::object::{load_object, validate_sha1, GitObject};
use anyhow::{anyhow, Result};
use std::io::{BufRead, BufReader, Cursor, Seek};

/// Given a hash of a file tree, loads the contents of the file where the file tree is stored.
/// Validates stored object type and prints the names of the files in the file tree.
pub fn ls_tree(hash: String) -> Result<()> {
    validate_sha1(hash.as_str())?;
    let GitObject {
        object_type, content
    } = load_object(hash)?;
    if object_type != "tree" {
        return Err(anyhow!("Expected object type 'tree' but got {}", object_type));
    }
    print_names(content)?;
    Ok(())
}

/// Given the content of a file tree, prints the names of the files in the tree.
fn print_names(content: Vec<u8>) -> Result<()> {
    let content_len = content.len();
    let mut reader = BufReader::new(Cursor::new(content));
    while reader.stream_position()? < content_len as u64 {

        // Read each file's flag
        let mut buffer = Vec::new();
        reader.read_until(' ' as u8, &mut buffer)?;
        buffer.pop();

        // Read each file's name
        buffer.clear();
        reader.read_until(0, &mut buffer)?;
        buffer.pop();

        let file_name = String::from_utf8(buffer)?;

        // Skip over file's hash (why is this hash 20 characters and not 40?)
        reader.seek_relative(20)?;

        // Print file name
        println!("{}", file_name);
    }
    Ok(())
}