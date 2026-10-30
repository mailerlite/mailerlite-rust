//! List recipient activity of a sent campaign: `GET /campaigns/{id}/reports/subscriber-activity`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_subscribers_activity_for_campaign
//! ```

use mailerlite_rs::{MailerLite, campaigns::ListCampaignActivity};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client
        .campaigns()
        .subscriber_activity("Your Campaign ID", &ListCampaignActivity::new().limit(25))
        .await?;

    println!("{page:#?}");

    Ok(())
}
