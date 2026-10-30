//! Verify and decode a signed webhook, then reject a modified body.
//!
//! Run with `cargo run --example verify_webhook --features webhook-signatures`.
//! Uses synthetic data and a public sample secret; no API token or server is needed.
use mailerlite_rs::webhooks::{SignatureError, WebhookEvent, WebhookPayload, verify_signature};

// Demonstration values only. An HTTP handler receives the body and Signature
// header from the request and loads its real signing secret from configuration.
const SECRET: &str = "offline-example-signing-secret";
const BODY: &[u8] = br#"{
  "event": "subscriber.created",
  "id": "123456789",
  "email": "jane@example.test",
  "status": "active",
  "sent": 0,
  "opens_count": 0,
  "clicks_count": 0,
  "open_rate": 0,
  "click_rate": 0,
  "created_at": "2026-01-01 12:00:00",
  "updated_at": "2026-01-01 12:00:00",
  "fields": {"name": "Jane"}
}"#;
// Computed independently with Python's hmac/hashlib over the exact bytes above.
const SIGNATURE: &str = "6799465b67058565b40081a6a6f9fef6773ec55676155adf267afa41bf8a5eed";

fn receive(
    secret: &str,
    signature: &str,
    body: &[u8],
) -> Result<WebhookPayload, Box<dyn std::error::Error>> {
    verify_signature(secret, body, signature)?;
    Ok(serde_json::from_slice(body)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payload = receive(SECRET, SIGNATURE, BODY)?;

    match payload {
        WebhookPayload::Single(WebhookEvent::SubscriberCreated(event)) => {
            println!(
                "Verified subscriber.created for sample subscriber {}.",
                event.subscriber.id
            );
        }
        _ => return Err("expected the sample subscriber.created event".into()),
    }

    // Even a whitespace change keeps the JSON valid but invalidates its signature.
    // Always verify the original request bytes before decoding or reformatting them.
    let mut tampered = BODY.to_vec();
    tampered.push(b' ');

    match receive(SECRET, SIGNATURE, &tampered) {
        Err(error)
            if matches!(
                error.downcast_ref::<SignatureError>(),
                Some(SignatureError::Mismatch)
            ) =>
        {
            println!("Rejected the modified body: signature mismatch.");
        }
        Err(error) => return Err(error),
        Ok(_) => return Err("modified body was unexpectedly accepted".into()),
    }

    Ok(())
}
