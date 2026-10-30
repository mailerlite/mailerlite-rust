//! Fetch the first page of subscriber groups.
//!
//! Requires `MAILERLITE_API_TOKEN`.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;

    let page = mailerlite.groups().list().page(1).limit(25).send().await?;

    for group in &page.data.data {
        println!("{}: {}", group.id, group.name);
    }

    println!("Next page: {:?}", page.data.next_page());

    Ok(())
}
