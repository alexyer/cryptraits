//! Signature traits.

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use crate::error::Error;

use super::convert::{FromBytes, ToVec};

pub trait Signature: FromBytes + ToVec + Copy + Clone + PartialEq {}

/// Sign a message.
pub trait Sign {
    type SIG: Signature;

    /// Sign a message.
    fn sign(&self, data: &[u8]) -> Self::SIG
    where
        Self: Sized;
}

/// Verify a signature
pub trait Verify {
    type E: Error;
    type SIG: Signature;

    /// Verify a signature
    fn verify(&self, data: &[u8], signature: &Self::SIG) -> Result<(), Self::E>;
}
