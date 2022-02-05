//! Stream cipher.

use zeroize::Zeroize;

use crate::error::Error;

/// Trait represents stream cipher.
pub trait StreamCipher: Zeroize {
    type E: Error;

    /// Create a new stream cipher instance from `key` and `nonce`.
    fn new_from_slices(key: &[u8], nonce: &[u8]) -> Result<Self, Self::E>
    where
        Self: Sized;

    /// Apply keystream to data.
    ///
    /// # Returns
    /// Error if the end of a keystream will be reached.
    fn apply_keystream(&mut self, data: &mut [u8]) -> Result<(), Self::E>;
}
