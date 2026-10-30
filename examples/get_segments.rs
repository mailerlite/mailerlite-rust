//! List segments: `GET /segments`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_segments
//! ```

use mailerlite_rs::{MailerLite, segments::ListSegments};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client
        .segments()
        .list(&ListSegments::new().limit(25))
        .await?;

    println!("{page:#?}");

    Ok(())
}
