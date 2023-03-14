use crate::object::GitObject;
use anyhow::{anyhow, Result};
use flate2::bufread::ZlibDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

/// Given a hash, opens the file stored in .git/objects in a directory named for the first 2
/// characters in the hash. Calls read_object to parse the file contents.
pub fn load_object(hash: String) -> Result<GitObject> {
    if hash.len() != 40 {
        return Err(anyhow!("Object id should be 40 characters but was {}", hash.len()));
    }
    // Determine sub-directory and file name from hash
    let sub_directory: String = hash.chars().take(2).collect();
    let file_name: String = hash.chars().skip(2).collect();
    let path = Path::new(".git")
        .join("objects")
        .join(sub_directory)
        .join(file_name);

    // Open file and create a ZLib decoder to read its contents
    let file = BufReader::new(File::open(path)?);
    let decoder = ZlibDecoder::new(file);

    read_object(decoder)
}

/// Given a file reader, reads in the header to determine the type of object stored in the file
/// and its content length. Verifies the content length and reads in the content to return a
/// GitObject containing the object's type and content.
fn read_object<R>(reader: R) -> Result<GitObject>
where R: Read,
{
   let mut reader = BufReader::new(reader);

    // Read in object type from header
    let mut buffer = Vec::new();
    reader.read_until(' ' as u8, &mut buffer)?;
    buffer.pop();
    let object_type = String::from_utf8(buffer.clone())?;

    // Read in object size from header
    buffer.clear();
    reader.read_until(0, &mut buffer)?;
    buffer.pop();

    let size = String::from_utf8(buffer.clone())?.parse::<usize>()?;

    // Read object content and verify length
    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;
    if content.len() != size {
        return Err(anyhow!("Incorrect content length, expected {} but was {}", size, content.len()));
    }

    // Return GitObject with object type and content
    Ok(GitObject {
        object_type,
        content
    })
}

