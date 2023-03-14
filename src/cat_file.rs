use crate::object::{load_object, validate_sha1, GitObject};
use anyhow::{anyhow, Result};
use std::io::{stdout, Cursor, Read};

/// Given a hash to a Git object file containing the header + content of a file,
/// print the file contents.
pub fn pretty_cat_file(hash: String) -> Result<()> {
    validate_sha1(hash.as_str())?;

    // Load the contents of the Git object file (header + content) into a Git object.
    let GitObject {
        object_type,
        content,
    } = load_object(hash)?;

    // Verify object type in case this function was given a hash for another type of Git object
    // such as a file tree or a commit.
    if object_type.as_str() != "blob" {
        return Err(anyhow!("Unsupported object type: {}", object_type));
    }

    // Print file contents by giving print_blob a Cursor into GitObject content vector,
    // since print_blob wants a Read type.
    print_blob(Cursor::new(content))
}

/// Print the contents of a file to stdout.
fn print_blob<R>(mut reader: R) -> Result<()>
where
    R: Read,
{
    // Simply copy the contents of the reader to stdout.
    std::io::copy(&mut reader, &mut stdout())?;
    Ok(())
}