use anyhow::{anyhow, Result};

/// Checks if a hash value is a valid SHA1 hash by checking if it is a hexadecimal value.
pub fn validate_sha1<T: AsRef<[u8]>>(hash: T) -> Result<()> {
    match hex::decode(hash) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Invalid sha: {}", e)),
    }
}