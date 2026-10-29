use mailerlite_rs::{
    api_response::ApiResponse,
    email::{
        BrandMode, BrandUpdate, EmailSettingsUpdate, ParagraphStyleUpdate, SettingsResult,
        TextStyleUpdate, TypographyUpdate,
    },
    MailerLite,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let email_id: String = String::from("Your Email ID");

    let etag: String = String::from("The etag from the last response");

    let settings: EmailSettingsUpdate = EmailSettingsUpdate {
        brand: Some(BrandUpdate {
            mode: Some(BrandMode::Custom.into()),
            background_color: Some(String::from("#f4f4f4")),
            ..Default::default()
        }),
        typography: Some(TypographyUpdate {
            paragraph: Some(ParagraphStyleUpdate {
                p1: Some(TextStyleUpdate {
                    size: Some(String::from("18")),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let response: ApiResponse<SettingsResult> = mailerlite
        .email()
        .update_settings(email_id, etag, &settings)
        .await?;

    println!("New etag: {}", response.etag);

    if let Some(rejected) = &response.rejected {
        println!("Removed values: {:?}", rejected.keys);
    }

    Ok(())
}
