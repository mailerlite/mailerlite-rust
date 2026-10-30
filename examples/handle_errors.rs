//! Handle API errors by kind: validation (422), rate limits (429), not found (404).
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example handle_errors
//! ```

use mailerlite_rs::{Error, MailerLite, subscribers::CreateSubscriber};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    match client
        .subscribers()
        .create(&CreateSubscriber::new("not-an-email"))
        .await
    {
        Ok(upserted) => println!("subscribed {}", upserted.data.email),
        Err(Error::Validation(invalid)) => {
            // One line, e.g. "MailerLite API error 422 Unprocessable Entity: email: ..."
            eprintln!("{invalid}");
            for error in invalid.iter() {
                eprintln!("  {}: {}", error.field(), error.message());
            }
        }
        Err(Error::RateLimited(limit)) => {
            eprintln!("rate limited, retry in {:?}", limit.retry_after());
        }
        Err(Error::NotFound(_)) => eprintln!("not found"),
        Err(other) => return Err(other),
    }

    Ok(())
}
