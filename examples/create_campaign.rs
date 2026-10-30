//! Create a regular campaign draft for a recipient group; this does not send it.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`, `MAILERLITE_SENDER_EMAIL`, `MAILERLITE_SENDER_NAME`, `MAILERLITE_GROUP_ID`.
use mailerlite_rs::{
    MailerLite,
    resources::campaigns::{CreateCampaign, Email},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let sender = std::env::var("MAILERLITE_SENDER_EMAIL")?;
    let sender_name = std::env::var("MAILERLITE_SENDER_NAME")?;
    let group = std::env::var("MAILERLITE_GROUP_ID")?;
    let email = Email::new("Monthly newsletter", sender_name, sender);
    let params = CreateCampaign::new("Monthly newsletter", email).groups([group]);
    let response = mailerlite.campaigns().create(params).await?;
    println!("Draft {}: {}", response.data.id, response.data.name);
    Ok(())
}
