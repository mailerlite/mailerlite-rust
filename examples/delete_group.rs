//! Delete a group: `DELETE /groups/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_group
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    client.groups().delete("Your Group ID").await?;

    println!("deleted");

    Ok(())
}
