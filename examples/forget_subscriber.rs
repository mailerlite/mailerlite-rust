//! Forget a subscriber (GDPR erasure): `POST /subscribers/{id}/forget`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example forget_subscriber
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let forgotten = client.subscribers().forget("Your Subscriber ID").await?;

    println!("{forgotten:#?}");

    Ok(())
}
