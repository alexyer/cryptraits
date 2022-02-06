//! Key management related traits.

#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(not(feature = "std"))]
use alloc::fmt::Debug;
use rand_core::{CryptoRng, RngCore};

#[cfg(feature = "std")]
use std::fmt::Debug;

#[cfg(not(feature = "std"))]
use alloc::string::String;

use zeroize::Zeroize;

use crate::error::Error;

/// Trait represents a public key.
pub trait PublicKey: Debug + Clone + Copy + PartialEq + Zeroize {}

/// Trait represents a secret key.
pub trait SecretKey: Clone + Zeroize + Debug {
    type PK: PublicKey;

    /// Generate an "unbiased" `SecretKey`;
    fn generate() -> Self;

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
pub trait KeyPair: Clone + Zeroize {
    type SK: SecretKey;

    /// Generate an "unbiased" `KeyPair`.
    fn generate() -> Self;

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

/// Generate and construct a value with mnemonic phrase and optional password.
pub trait WithPhrase {
    type E: Error;

    /// Generate a new value of `word_count` words and optional password.
    ///
    /// Returns tuple of generated value and a phrase or error.
    fn generate_with_phrase(
        word_count: usize,
        password: Option<&str>,
    ) -> Result<(Self, String), Self::E>
    where
        Self: Sized;

    /// Construct a value from mnemonic phrase and optional password.
    fn from_phrase<'a, S: Into<Cow<'a, str>>>(
        s: S,
        password: Option<&str>,
    ) -> Result<Self, Self::E>
    where
        Self: Sized;

    /// Generate a new value of `word_count` words and optional password witn `rng` PRF.
    ///
    /// Returns tuple of generated value and a phrase or error.
    fn generate_in_with<R>(
        rng: &mut R,
        word_count: usize,
        password: Option<&str>,
    ) -> Result<(Self, String), Self::E>
    where
        Self: Sized,
        R: RngCore + CryptoRng;
}

/// Perform a blinding operation on keys.
pub trait Blind {
    type E: Error;

    /// Perform a blinding operation on the key with the given blinding factor.
    fn blind(&mut self, blinding_factor: &[u8]) -> Result<(), Self::E>;

    /// Perform a blinding operation on the key with the given blinding factor.
    fn to_blind(&self, blinding_factor: &[u8]) -> Result<Self, Self::E>
    where
        Self: Sized;
}
