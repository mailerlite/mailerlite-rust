//! Run several API calls in one request: `POST /batch`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_batch
//! ```

use mailerlite_rs::{MailerLite, batch::BatchRequest};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let request = BatchRequest::new()
        .get("api/groups?limit=1")
        .get("api/subscribers?limit=1");
    let response = client.batch().send(&request).await?;

    println!("{response:#?}");

    Ok(())
}
