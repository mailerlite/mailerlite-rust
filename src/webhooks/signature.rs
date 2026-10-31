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
/// The function does not parse JSON or validate the event schema.
///
/// # Examples
///
/// Enable `webhook-signatures` and add `serde_json` to your application's
/// dependencies. Your HTTP handler supplies the signing secret, original body
/// bytes, and `Signature` header; reject a missing header before calling this.
///
/// ```
/// use mailerlite_rs::webhooks::{WebhookPayload, verify_signature};
///
/// fn receive(
///     secret: &str,
///     body: &[u8],
///     signature: &str,
/// ) -> Result<WebhookPayload, Box<dyn std::error::Error>> {
///     verify_signature(secret, body, signature)?;
///
///     Ok(serde_json::from_slice(body)?)
/// }
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Synthetic, independently signed sample; never use this secret in production.
/// let body = br#"{"event":"future.event","note":"raw bytes"}"#;
/// let signature = "1b7c95ba98c90ff8e90d9206c689dfa17bb0c42b7b9d542a3a2a8fa58bad53c6";
///
/// let payload = receive("test-webhook-secret", body, signature)?;
/// # let _ = payload;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns [`SignatureError::EmptySecret`] for an empty signing secret,
/// [`SignatureError::InvalidEncoding`] unless the header is exactly 64
/// hexadecimal characters, or [`SignatureError::Mismatch`] if the tag does not
/// authenticate the supplied body and secret. Reject the delivery on any of
/// these errors. A valid signature does not guarantee valid JSON; subsequent
/// payload decoding can fail separately.
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
