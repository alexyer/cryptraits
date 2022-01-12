//! AEAD cipher.

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use crate::error::Error;

/// Trait represents AEAD cipher.
pub trait Aead {
    /// AEAD Error type.
    type E: Error;

    /// Nonce length in bytes.
    const NONCE_LEN: usize;

    /// Creates a new cipher instance.
    fn new(key: &[u8]) -> Self;

    /// Encrypts `data` with `nonce`.
    fn encrypt(&self, nonce: &[u8], data: &[u8], aad: Option<&[u8]>) -> Result<Vec<u8>, Self::E>;

    /// Decrypts `data` with `nonce`.
    fn decrypt(&self, nonce: &[u8], data: &[u8], aad: Option<&[u8]>) -> Result<Vec<u8>, Self::E>;
}
