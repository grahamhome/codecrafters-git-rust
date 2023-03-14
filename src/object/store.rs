use crate::object::GitObject;
use anyhow::Result;
use flate2::bufread::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs::{create_dir, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;

/// Given a tree or blob object, generates the header + content string,
/// hashes it, creates a file to hold the ZLib-encoded header + content,
/// and writes the ZLib-encoded header + content to the file.
pub fn store_object(mut object: GitObject) -> Result<String> {
    // Create a buffer to hold the data to be hashed -
    // object type, object content length and object content.
    let mut buffer = Vec::new();

    // Write the header - object type, space, object content length, null
    buffer.extend(object.object_type.as_bytes());
    buffer.extend(" ".as_bytes());
    buffer.extend(object.content.len().to_string().as_bytes());
    buffer.push(0);

    // Write the object's content
    buffer.append(&mut object.content);

    // Hash the header + content
    let hash = calculate_sha1(&mut buffer);

    // Ensure output file exists for object's content based on object's hash
    let output_file = create_output_file(&hash)?;

    // Create a ZLib encoder for the header + content
    let mut zlib_reader = ZlibEncoder::new(BufReader::new(&buffer[..]), Compression::fast());

    // Write the ZLib-encoded header + content to the file.
    std::io::copy(&mut zlib_reader, &mut BufWriter::new(output_file))?;

    // Return the hash.
    Ok(hash)
}

/// Returns the SHA1 hash of the given header + content.
fn calculate_sha1(buffer: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&buffer);
    hex::encode(hasher.finalize())
}

/// Given a hash of a blob or a tree, creates a directory under .git/objects from the
/// first 2 characters of the hash and creates a file inside this directory named with the
/// remaining letters of the hash. Does not write any content to the file.
fn create_output_file(hash: &String) -> Result<File> {
    let sub_directory: String = hash.chars().take(2).collect();
    let file_name: String = hash.chars().skip(2).collect();
    let mut output_path = Path::new(".git").join("objects").join(sub_directory);
    if !output_path.exists() {
        create_dir(output_path.clone())?;
    }
    output_path = output_path.join(file_name);
    let file = File::create(output_path)?;
    Ok(file)
}