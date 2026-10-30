//! Rename an existing custom field.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_FIELD_ID`, `MAILERLITE_FIELD_NAME`.

use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::fields::UpdateField;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_FIELD_ID")?;
    let name = std::env::var("MAILERLITE_FIELD_NAME")?;
    let field = mailerlite
        .fields()
        .update(id, UpdateField::new(name))
        .await?;
    println!("Updated field: {} ({})", field.data.id, field.data.name);
    Ok(())
}
