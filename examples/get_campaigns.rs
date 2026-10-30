//! List campaigns: `GET /campaigns`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_campaigns
//! ```

use mailerlite_rs::{
    MailerLite,
    campaigns::{CampaignStatus, ListCampaigns},
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client
        .campaigns()
        .list(&ListCampaigns::new().status(CampaignStatus::Sent).limit(25))
        .await?;

    println!("{page:#?}");

    Ok(())
}
