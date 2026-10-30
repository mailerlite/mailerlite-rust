//! Update a draft campaign: `PUT /campaigns/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_campaign
//! ```

use mailerlite_rs::{
    MailerLite,
    campaigns::{CampaignEmail, UpdateCampaign},
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let email = CampaignEmail::new(
        "Updated subject",
        "Your Name",
        "you@your-verified-domain.com",
    );
    let campaign = client
        .campaigns()
        .update(
            "Your Campaign ID",
            &UpdateCampaign::new("Renamed campaign", email),
        )
        .await?;

    println!("{campaign:#?}");

    Ok(())
}
