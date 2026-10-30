//! Rename a field: `PUT /fields/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_field
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let field = client
        .fields()
        .update("Your Field ID", "Renamed field")
        .await?;

    println!("{field:#?}");

    Ok(())
}
