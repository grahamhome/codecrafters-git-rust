mod hash;
mod load;
mod store;

pub use hash::validate_sha1;
pub use load::load_object;
pub use store::store_object;

/// Represents a file or a file tree within a Git repo.
pub struct GitObject {
    // "blob" or "tree"
    pub object_type: String,
    // file or tree contents
    pub content: Vec<u8>,
}