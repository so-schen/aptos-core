// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! This file implements traits for WebAuthn Account Authenticators

use crate::{
    ed25519::Ed25519Signature,
    hash::CryptoHash,
    p256::{P256Signature, P256_SIGNATURE_LENGTH},
    traits::*,
    webauthn::WebAuthnPublicKey,
};
use anyhow::{anyhow, Result};
use aptos_crypto_derive::{DeserializeKey, SerializeKey};
use serde::Serialize;
use std::fmt;

impl private::Sealed for WebAuthnSignature {}

/// This struct contains other WebAuthn fields for verification
#[derive(Clone, Serialize)]
pub struct WebAuthnFields {
    client_data_json: Vec<u8>,
    authenticator_data: Vec<u8>,
}

/// This file implements traits for WebAuthn signatures
#[derive(Clone, Serialize)]
pub enum WebAuthnSignature {
    /// P256 Signatures with WebAuthn
    P256Signature {
        /// P256 Signatures
        signature: P256Signature,
        /// WebAuthn Fields
        web_authn_fields: WebAuthnFields,
    },
    /// Ed25519 Signatures with WebAuthn
    Ed25519Signature {
        /// Ed25519 Signatures
        signature: Ed25519Signature,
        /// WebAuthn Fields
        web_authn_fields: WebAuthnFields,
    },
}

impl WebAuthnSignature {
    // /// The length of the P256Signature
    // pub const LENGTH: usize = P256_SIGNATURE_LENGTH;
    //
    // /// Serialize a P256Signature.
    // pub fn to_bytes(&self) -> [u8; P256_SIGNATURE_LENGTH] {
    //     match self {
    //         WebAuthnSignature::P256Signature { signature, .. } => {
    //             signature.0.to_bytes()
    //         }
    //         WebAuthnSignature::Ed25519Signature { signature, .. } => {
    //             signature.0.to_bytes()
    //         }
    //     }
    //     self.0.to_bytes()
    // }
}

//////////////////////
// Signature Traits //
//////////////////////

// impl Signature for WebAuthnSignature {
impl WebAuthnSignature {
    // type SigningKeyMaterial = P256PrivateKey;
    // type VerifyingKeyMaterial = P256PublicKey;

    /// Verifies that the provided signature is valid for the provided message, going beyond the
    /// [RFC8032](https://tools.ietf.org/html/rfc8032) specification, checking both scalar
    /// malleability and point malleability (see documentation [here](https://docs.rs/ed25519-dalek/latest/ed25519_dalek/struct.PublicKey.html#on-the-multiple-sources-of-malleability-in-ed25519-signatures)).
    ///
    /// This _strict_ verification performs steps 1,2 and 3 from Section 5.1.7 in RFC8032, and an
    /// additional scalar malleability check (via [Ed25519Signature::check_s_malleability][Ed25519Signature::check_s_malleability]).
    ///
    /// This function will ensure both the signature and the `public_key` are not in a small subgroup.
    ///
    /// TODO: this needs to verify two things
    ///
    /// 1. The signature is correct (this is done via verify_arbitrary_msg)
    /// 2. The (authenticatorData || H(clientDataJSON)) === message
    ///
    /// Also function isn't public when implemented with Signature trait (should change back to private)
    pub fn verify<T: CryptoHash + Serialize>(
        &self,
        message: &T,
        public_key: &WebAuthnPublicKey,
    ) -> Result<()> {
        Self::verify_arbitrary_msg(self, &signing_message(message)?, public_key)
    }

    /// Checks that `self` is valid for an arbitrary &[u8] `message` using `public_key`.
    /// Outside of this crate, this particular function should only be used for native signature
    /// verification in Move.
    ///
    /// This function will check both the signature and `public_key` for small subgroup attacks.
    fn verify_arbitrary_msg(&self, message: &[u8], public_key: &WebAuthnPublicKey) -> Result<()> {
        match public_key {
            WebAuthnPublicKey::Ed25519 { public_key } => {
                match self {
                    /// TODO: Make error message better
                    WebAuthnSignature::P256Signature { .. } => Err(anyhow!("Incorrect scheme")),
                    WebAuthnSignature::Ed25519Signature { signature, .. } => public_key
                        .0
                        .verify_strict(message, &signature.0)
                        .map_err(|e| anyhow!("{}", e))
                        .and(Ok(())),
                }
            },
            WebAuthnPublicKey::P256 { public_key } => {
                /// TODO: No verify_strict on p256 pubkey???
                // public_key
                //     .verify_strict(message, &self.0)
                //     .map_err(|e| anyhow!("{}", e))
                //     .and(Ok(()))
                Ok(())
            },
        }
    }

    /// TODO: In need of Deserialize macro as well?
    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}

impl Length for WebAuthnSignature {
    fn length(&self) -> usize {
        P256_SIGNATURE_LENGTH
    }
}

// impl ValidCryptoMaterial for WebAuthnSignature {
//     fn to_bytes(&self) -> Vec<u8> {
//         self.to_bytes().to_vec()
//     }
// }

impl std::hash::Hash for WebAuthnSignature {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let encoded_signature = self.to_bytes();
        state.write(&encoded_signature);
    }
}

// impl TryFrom<&[u8]> for WebAuthnSignature {
//     type Error = CryptoMaterialError;
//
//     fn try_from(bytes: &[u8]) -> std::result::Result<WebAuthnSignature, CryptoMaterialError> {
//         P256Signature::check_s_malleability(bytes)?;
//         P256Signature::from_bytes_unchecked(bytes)
//     }
// }

// Those are required by the implementation of hash above
impl PartialEq for WebAuthnSignature {
    fn eq(&self, other: &WebAuthnSignature) -> bool {
        self.to_bytes()[..] == other.to_bytes()[..]
    }
}

impl Eq for WebAuthnSignature {}

impl fmt::Display for WebAuthnSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.to_bytes()[..]))
    }
}

impl fmt::Debug for WebAuthnSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WebAuthn Signature({})", self)
    }
}
