//! Update email settings.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_EMAIL_ID`, `MAILERLITE_EMAIL_ETAG`,
//! `MAILERLITE_BACKGROUND_COLOR`, `MAILERLITE_PARAGRAPH_SIZE`.
//! Sets custom brand mode and replaces the background color and paragraph size.
use mailerlite_rs::{
    MailerLite, Response,
    models::emails::{BrandMode, EmailResponseMetadata, SettingsResult},
    resources::emails::{
        BrandUpdate, EmailSettingsUpdate, ParagraphStyleUpdate, TextStyleUpdate, TypographyUpdate,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let etag: String = std::env::var("MAILERLITE_EMAIL_ETAG")?;

    let settings: EmailSettingsUpdate = EmailSettingsUpdate {
        brand: Some(BrandUpdate {
            mode: Some(BrandMode::Custom),
            background_color: Some(std::env::var("MAILERLITE_BACKGROUND_COLOR")?),
            ..Default::default()
        }),
        typography: Some(TypographyUpdate {
            paragraph: Some(ParagraphStyleUpdate {
                p1: Some(TextStyleUpdate {
                    size: Some(std::env::var("MAILERLITE_PARAGRAPH_SIZE")?),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let response: Response<SettingsResult, EmailResponseMetadata> = mailerlite
        .emails()
        .update_settings(email_id, etag, &settings)
        .await?;

    println!("New etag: {}", response.data.etag);

    if let Some(rejected) = &response.data.rejected {
        println!("Removed values: {:?}", rejected.keys);
    }

    Ok(())
}
