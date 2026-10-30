//! Create a draft campaign: `POST /campaigns`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_campaign
//! ```

use mailerlite_rs::{
    MailerLite,
    campaigns::{CampaignEmail, CreateCampaign},
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let email = CampaignEmail::new("Hello!", "Your Name", "you@your-verified-domain.com")
        .content("<p>Hello {$name}</p><p><a href=\"{$unsubscribe}\">Unsubscribe</a></p>");
    let campaign = client
        .campaigns()
        .create(&CreateCampaign::regular("New campaign", email).groups(["Your Group ID"]))
        .await?;

    println!("{campaign:#?}");

    Ok(())
}
