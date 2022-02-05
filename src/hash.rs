//! Hash.

use zeroize::Zeroize;

/// Trait represents Hash.
pub trait Hash: Zeroize {
    /// Create new hasher.
    fn new() -> Self;

    /// Update state using the provided data.
    fn update(&mut self, data: &[u8]);

    /// Retrieve result and consume hasher instance.
    fn finalize(self) -> Vec<u8>;
}
