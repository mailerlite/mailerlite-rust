//! Verification of incoming webhook signatures against original body bytes.
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

/// A webhook signature could not be verified. Values and secrets are never retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum SignatureError {
    /// A nonempty webhook signing secret must be configured.
    #[error("webhook signing secret must not be empty")]
    EmptySecret,
    /// The Signature header must contain exactly 64 hexadecimal characters.
    #[error("webhook signature must contain exactly 64 hexadecimal characters")]
    InvalidEncoding,
    /// The signature does not authenticate the supplied body and secret.
    #[error("webhook signature verification failed")]
    Mismatch,
}

/// Verify MailerLite's `Signature` header using the webhook secret and raw body.
///
/// Pass the original request bytes, including their whitespace and encoding;
/// never parse and reserialize JSON before verification. The supplied secret is
/// the webhook's signing secret, not the MailerLite API token. Tag comparison
/// uses the HMAC implementation's constant-time verification. A valid signature
/// authenticates the bytes; it does not establish freshness or deduplicate a
/// repeated delivery.
///
/// Header values must contain exactly 64 hexadecimal characters. Prefixes,
/// whitespace and truncated tags are rejected. Both hexadecimal cases work.
pub fn verify_signature(secret: &str, body: &[u8], signature: &str) -> Result<(), SignatureError> {
    if secret.is_empty() {
        return Err(SignatureError::EmptySecret);
    }
    if signature.len() != 64 {
        return Err(SignatureError::InvalidEncoding);
    }
    let mut tag = [0_u8; 32];
    hex::decode_to_slice(signature, &mut tag).map_err(|_| SignatureError::InvalidEncoding)?;
    // HMAC accepts arbitrary key lengths. Keep construction fallible even if a
    // future dependency changes that guarantee; no recoverable failure panics.
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).map_err(|_| SignatureError::Mismatch)?;
    mac.update(body);
    mac.verify_slice(&tag).map_err(|_| SignatureError::Mismatch)
}
