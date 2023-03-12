use crate::object::{load_object, validate_sha1, GitObject};
use anyhow::{anyhow, Result};
use std::io::{BufRead, BufReader, Cursor, Seek};

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

fn print_names(content: Vec<u8>) -> Result<()> {
    let content_len = content.len();
    let mut reader = BufReader::new(Cursor::new(content));
    while reader.stream_position()? < content_len as u64 {
        let mut buffer = Vec::new();
        reader.read_until(' ' as u8, &mut buffer);
        buffer.pop();

        buffer.clear();
        reader.read_until(0, &mut buffer);
        buffer.pop();

        let file_name = String::from_utf8(buffer)?;

        reader.seek_relative(20)?;

        println!("{}", file_name);
    }
    Ok(())
}