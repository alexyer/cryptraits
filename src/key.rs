//! Key management related traits.

#[cfg(not(feature = "std"))]
use alloc::fmt::Debug;
use rand_core::{CryptoRng, RngCore};

#[cfg(feature = "std")]
use std::fmt::Debug;

use zeroize::Zeroize;

/// Trait represents a public key.
pub trait PublicKey: Debug + Copy + PartialEq {}

/// Trait represents a secret key.
pub trait SecretKey: Zeroize + Debug {
    type PK: PublicKey;

    /// Generates an "unbiased" `SecretKey` directly from a user
    /// suplied `csprng` uniformly.
    fn generate_with<R: CryptoRng + RngCore>(csprng: R) -> Self
    where
        Self: Sized;

    /// Derives the `PublicKey` corresponding to this `SecretKey`.
    fn to_public(&self) -> Self::PK;
}

/// Trait represents a shared secret key (e.g. obtained via DH exchange).
pub trait SharedSecretKey {}

/// Trait represents a keypair.
pub trait KeyPair: Zeroize {
    type SK: SecretKey;

    /// Generates an "unbiased" `KeyPair` directly from a user
    /// suplied `csprng` uniformly.
    fn generate_with<R>(csprng: R) -> Self
    where
        R: CryptoRng + RngCore;

    /// Get a `PublicKey` of `KeyPair`.
    fn public(&self) -> &<Self::SK as SecretKey>::PK;

    /// Derives the `PublicKey` corresponding to `KeyPair` `SK`;
    fn to_public(&self) -> <Self::SK as SecretKey>::PK;

    /// Get a `SecretKey` of `KeyPair`.
    fn secret(&self) -> &Self::SK;
}
