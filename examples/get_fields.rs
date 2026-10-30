//! Fetch the first page of custom fields.
//!
//! Requires `MAILERLITE_API_TOKEN`.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;

    let page = mailerlite.fields().list().page(1).limit(25).send().await?;

    for field in &page.data.data {
        println!("{}: {} ({})", field.id, field.name, field.field_type);
    }

    println!("Next page: {:?}", page.data.next_page());

    Ok(())
}
