//! Verifying the `Signature` header MailerLite sends with every webhook.
//!
//! The header carries `hex(HMAC-SHA256(raw_request_body, webhook_secret))`; the secret is
//! returned when you create a webhook ([`Webhook::secret`](crate::webhooks::Webhook::secret)).

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

/// Returns `true` if `signature` (the hex `Signature` header) matches `body` signed with
/// `secret`. The comparison is constant-time.
///
/// Pass the raw request body bytes exactly as received, before any JSON parsing.
///
/// ```
/// use mailerlite_rs::webhook_signature::verify;
///
/// fn handle(raw_body: &[u8], signature_header: &str, secret: &str) -> Result<(), &'static str> {
///     if !verify(raw_body, signature_header, secret) {
///         return Err("invalid signature");
///     }
///     // Safe to parse `raw_body` now.
///     Ok(())
/// }
/// ```
#[must_use]
pub fn verify(body: &[u8], signature: &str, secret: &str) -> bool {
    let Some(expected) = decode_hex(signature.trim()) else {
        return false;
    };
    let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(body);
    mac.verify_slice(&expected).is_ok()
}

fn decode_hex(hex: &str) -> Option<Vec<u8>> {
    fn nibble(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }

    let bytes = hex.as_bytes();
    if bytes.len() % 2 != 0 {
        return None;
    }
    bytes
        .chunks_exact(2)
        .map(|pair| Some(nibble(pair[0])? << 4 | nibble(pair[1])?))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &[u8] = br#"{"event":"subscriber.created"}"#;
    /// Well-formed hex that is not the HMAC of `BODY`.
    const FORGED: &str = "4a1f3b2d8c1e2b2a35b7d8d9a2dd8e0e8d0d6a62b0f31d9b1b8c0d1f6b8f9c51";

    fn sign(body: &[u8], secret: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body);
        mac.finalize()
            .into_bytes()
            .iter()
            .fold(String::new(), |mut hex, byte| {
                use std::fmt::Write as _;
                let _ = write!(hex, "{byte:02x}");
                hex
            })
    }

    #[test]
    fn accepts_valid_signature_in_any_case() {
        let signature = sign(BODY, "secret");
        assert!(verify(BODY, &signature, "secret"));
        assert!(verify(BODY, &signature.to_uppercase(), "secret"));
    }

    #[test]
    fn rejects_tampering_and_garbage() {
        let signature = sign(BODY, "secret");
        assert!(!verify(b"{}", &signature, "secret"));
        assert!(!verify(BODY, &signature, "other"));
        assert!(!verify(BODY, FORGED, "secret"));
        assert!(!verify(BODY, "zz", "secret"));
        assert!(!verify(BODY, "abc", "secret"));
        assert!(!verify(BODY, "", "secret"));
    }
}
