//! Key exchange traits.

use super::key::{PublicKey, SharedSecretKey};

/// Diffie-Hellman key exchange.
pub trait DiffieHellman {
    type SSK: SharedSecretKey;
    type PK: PublicKey;

    /// Derives `SharedSecretKey` from the other `PublicKey`
    fn diffie_hellman(&self, peer_public: &Self::PK) -> Self::SSK;
}
