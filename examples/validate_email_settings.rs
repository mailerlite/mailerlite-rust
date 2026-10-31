//! Validate email settings. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{
    MailerLite, Response,
    models::emails::Validation,
    resources::emails::{ButtonUpdate, EmailSettingsUpdate},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let settings: EmailSettingsUpdate = EmailSettingsUpdate {
        button: Some(ButtonUpdate {
            border_radius: Some(String::from("8")),
            ..Default::default()
        }),
        ..Default::default()
    };

    let response: Response<Validation> = mailerlite
        .emails()
        .validate_settings(email_id, &settings)
        .await?;

    if !response.data.valid {
        println!("{:#?}", response.data.errors);
    }

    Ok(())
}
