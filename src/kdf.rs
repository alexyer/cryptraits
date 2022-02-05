//! KDF trait.

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
use zeroize::Zeroize;

use crate::error::Error;

/// Trait represents Key Derivation Function.
pub trait Kdf: Zeroize {
    type E: Error;

    /// Creates a new KDF instance.
    fn new(salt: Option<&[u8]>, data: &[u8]) -> Self;

    /// Derives bytes from `info`.
    fn expand(&self, info: &[u8], okm: &mut [u8]) -> Result<(), Self::E>;
}
