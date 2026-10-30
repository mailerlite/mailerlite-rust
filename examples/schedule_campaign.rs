//! Schedule a campaign: `POST /campaigns/{id}/schedule`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example schedule_campaign
//! ```

use mailerlite_rs::{
    MailerLite,
    campaigns::{Schedule, ScheduleCampaign},
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let campaign = client
        .campaigns()
        .schedule(
            "Your Campaign ID",
            &ScheduleCampaign::scheduled(Schedule::new("2030-01-01", "09", "00")),
        )
        .await?;

    println!("{campaign:#?}");

    Ok(())
}
