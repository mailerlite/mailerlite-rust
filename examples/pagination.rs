//! Manually page through subscribers without collecting all pages.
//!
//! Requires `MAILERLITE_API_TOKEN`.
use mailerlite_rs::{MailerLite, models::subscribers::SubscriberStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let subscribers = mailerlite.subscribers();
    let mut cursor = None;
    loop {
        let mut request = subscribers
            .list()
            .status(SubscriberStatus::Active)
            .limit(100);
        if let Some(value) = cursor {
            request = request.cursor(value);
        }
        let response = request.send().await?;
        for subscriber in &response.data.data {
            println!("{}", subscriber.id);
        }
        cursor = response.data.next_cursor().map(str::to_owned);
        if cursor.is_none() {
            break;
        }
    }
    Ok(())
}
