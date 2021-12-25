use super::*;

// A hashable type.
// Hashable is a trait that allows you to hash a type.
pub trait Hashable {
    // Convert as bytes.
    fn bytes (&self) -> Vec<u8>;

    // Convert the type into a hash.
    // Algorithm: SHA256
    fn hash (&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}