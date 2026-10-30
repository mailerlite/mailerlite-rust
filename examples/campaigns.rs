//! Campaigns: create a draft, update it, schedule, cancel, report and delete.
//!
//! ```sh
//! MAILERLITE_API_KEY=... MAILERLITE_SENDER=you@verified-domain.com cargo run --example campaigns
//! ```
//!
//! Runs against your real account. The sender address must be verified in MailerLite. The
//! campaign is scheduled for a date far in the future, cancelled straight away and deleted.

use mailerlite_rs::{
    MailerLite,
    campaigns::{
        CampaignEmail, CampaignStatus, CreateCampaign, ListCampaignActivity, ListCampaigns,
        Schedule, ScheduleCampaign, UpdateCampaign,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;
    let sender = std::env::var("MAILERLITE_SENDER")?;
    let campaigns = client.campaigns();

    // The list endpoint only returns `ready` campaigns unless you ask for another status.
    let sent = campaigns
        .list(&ListCampaigns::new().status(CampaignStatus::Sent).limit(5))
        .await?;
    for campaign in &sent.data {
        println!("sent: {} ({})", campaign.name, campaign.id);
    }

    let email = CampaignEmail::new("Hello from Rust", "Rust SDK", sender.as_str()).content(
        "<p>Hi {$name}, this is a test.</p><p><a href=\"{$unsubscribe}\">Unsubscribe</a></p>",
    );
    let campaign = campaigns
        .create(&CreateCampaign::regular("Rust SDK example", email.clone()))
        .await?;
    println!("created draft {}", campaign.id);

    let campaign = campaigns
        .update(
            &campaign.id,
            &UpdateCampaign::new("Rust SDK example (edited)", email),
        )
        .await?;

    let scheduled = campaigns
        .schedule(
            &campaign.id,
            &ScheduleCampaign::scheduled(Schedule::new("2099-12-31", "09", "00")),
        )
        .await?;
    println!("scheduled: {:?}", scheduled.status);

    let draft = campaigns.cancel(&campaign.id).await?;
    println!("back to {:?}", draft.status);

    if let Some(last) = sent.data.first() {
        let activity = campaigns
            .subscriber_activity(&last.id, &ListCampaignActivity::new().limit(10))
            .await?;
        println!(
            "{} recipients on the first activity page",
            activity.data.len()
        );
    }

    for language in campaigns.languages().await?.iter().take(3) {
        println!("language {}: {}", language.id, language.name);
    }

    campaigns.delete(&campaign.id).await?;
    println!("deleted {}", campaign.id);

    Ok(())
}
