//! Hash-based Message Authentication Code (HMAC).
use crate::error::Error;

/// Trait represents HMAC.
pub trait Hmac {
    /// HMAC Error type.
    type E: Error;

    /// Create new value from variable size key.
    fn new_from_slice(key: &[u8]) -> Result<Self, Self::E>
    where
        Self: Sized;

    /// Update state using the provided data.
    fn update(&mut self, data: &[u8]);

    /// Check truncated tag correctness using all bytes of calculated tag.
    fn verify_slice(self, tag: &[u8]) -> Result<(), Self::E>;

    /// Obtain the result of a MAC computation as a `Vec` and consume MAC instance.
    fn finalize(self) -> Vec<u8>;
}
