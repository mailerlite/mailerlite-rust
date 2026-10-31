//! Create a text custom field.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_FIELD_NAME`.

use mailerlite_rs::MailerLite;
use mailerlite_rs::{models::fields::FieldType, resources::fields::CreateField};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let name = std::env::var("MAILERLITE_FIELD_NAME")?;

    let field = mailerlite
        .fields()
        .create(CreateField::new(name, FieldType::Text))
        .await?;

    println!("Created field: {} (key: {})", field.data.id, field.data.key);

    Ok(())
}
