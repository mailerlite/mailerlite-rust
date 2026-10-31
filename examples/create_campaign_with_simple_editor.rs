//! Create a simple-editor draft using explicit campaign and sender values.
use mailerlite_rs::{
    MailerLite,
    resources::campaigns::{CreateCampaign, Email},
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;
    let email = Email::new(
        std::env::var("MAILERLITE_SUBJECT")?,
        std::env::var("MAILERLITE_FROM_NAME")?,
        std::env::var("MAILERLITE_FROM_EMAIL")?,
    );
    let draft =
        CreateCampaign::new(std::env::var("MAILERLITE_CAMPAIGN_NAME")?, email).simple_editor();
    let response = client.campaigns().create(draft).await?;
    println!("Default email ID: {:?}", response.data.default_email_id);
    Ok(())
}
