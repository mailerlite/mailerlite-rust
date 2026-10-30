//! Verify the `Signature` header of an incoming webhook (`webhook-signature` feature).
//!
//! ```sh
//! cargo run --example verify_webhook_signature --features webhook-signature
//! ```

use mailerlite_rs::webhook_signature::verify;

fn main() {
    // In your web handler: the raw request body, the `Signature` header and the webhook's
    // `secret` (returned when the webhook was created).
    let raw_body = br#"{"events":[],"total":0}"#;
    let signature_header = "value of the Signature header";
    let secret = "Your Webhook Secret";

    if verify(raw_body, signature_header, secret) {
        println!("valid: safe to parse the body");
    } else {
        println!("invalid signature: reject the request");
    }
}
