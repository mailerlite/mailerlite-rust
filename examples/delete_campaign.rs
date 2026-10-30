//! Delete a campaign: `DELETE /campaigns/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_campaign
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    client.campaigns().delete("Your Campaign ID").await?;

    println!("deleted");

    Ok(())
}
