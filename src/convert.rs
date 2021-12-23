//! Convertion traits.

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use crate::error::Error;

/// Convert from bytes.
pub trait FromBytes {
    type E: Error;
    const LEN: usize;

    /// Construct a value from a slice of bytes.
    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::E>
    where
        Self: Sized;
}

/// Convert value into `Vec` of bytes.
pub trait ToVec {
    const LEN: usize;

    /// Convert a key into a vec of bytes.
    fn to_vec(&self) -> Vec<u8>
    where
        Self: Sized;
}
