//! Delete an existing custom field.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_FIELD_ID`.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_FIELD_ID")?;
    mailerlite.fields().delete(&id).await?;
    println!("Deleted field: {id}");
    Ok(())
}
