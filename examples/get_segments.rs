//! Fetch the first page of subscriber segments.
//!
//! Requires `MAILERLITE_API_TOKEN`.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let page = mailerlite
        .segments()
        .list()
        .page(1)
        .limit(25)
        .send()
        .await?;
    for segment in &page.data.data {
        println!("{}: {}", segment.id, segment.name);
    }
    println!("Next page: {:?}", page.data.next_page());
    Ok(())
}
