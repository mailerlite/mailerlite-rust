//! Webhooks: create, update, list and delete.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example webhooks
//! ```
//!
//! Verify incoming deliveries with `mailerlite_rs::webhook_signature::verify` (feature
//! `webhook-signature`) using the webhook's `secret`.

use mailerlite_rs::{
    MailerLite,
    webhooks::{CreateWebhook, ListWebhooks, UpdateWebhook, WebhookEvent},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;
    let webhooks = client.webhooks();

    let webhook = webhooks
        .create(
            &CreateWebhook::new(
                "https://example.com/mailerlite/webhook",
                [
                    WebhookEvent::SubscriberCreated,
                    WebhookEvent::SubscriberUnsubscribed,
                ],
            )
            .name("Rust SDK example"),
        )
        .await?;
    println!(
        "created webhook {}; store its secret to verify signatures",
        webhook.id
    );

    // `campaign.open`, `campaign.click` and `subscriber.deleted` require batchable webhooks.
    webhooks
        .update(
            &webhook.id,
            &UpdateWebhook::new()
                .events([WebhookEvent::CampaignOpen])
                .batchable(true),
        )
        .await?;

    for webhook in webhooks.list(&ListWebhooks::new()).await? {
        println!(
            "{} -> {} ({} events)",
            webhook.id,
            webhook.url,
            webhook.events.len()
        );
    }

    webhooks.delete(&webhook.id).await?;
    Ok(())
}
