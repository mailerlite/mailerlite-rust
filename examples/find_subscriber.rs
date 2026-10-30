//! Get a subscriber by id or email: `GET /subscribers/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example find_subscriber
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let subscriber = client
        .subscribers()
        .get("Your Subscriber ID or email")
        .await?;

    println!("{subscriber:#?}");

    Ok(())
}
