//! Inspect an import report. Requires MAILERLITE_API_TOKEN and MAILERLITE_IMPORT_ID.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_IMPORT_ID")?;
    let response = mailerlite.imports().get(id).await?;
    println!("Import done: {}", response.data.done);
    if let Some(count) = response.data.invalid_count {
        println!("Invalid inputs: {count}");
    }
    if let Some(samples) = response.data.invalid {
        println!("Invalid input samples returned: {}", samples.len());
    }
    Ok(())
}
