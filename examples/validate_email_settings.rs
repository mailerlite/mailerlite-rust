use mailerlite_rs::{
    api_response::ApiResponse,
    email::{ButtonUpdate, EmailSettingsUpdate, Validation},
    MailerLite,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let email_id: String = String::from("Your Email ID");

    let settings: EmailSettingsUpdate = EmailSettingsUpdate {
        button: Some(ButtonUpdate {
            border_radius: Some(String::from("8")),
            ..Default::default()
        }),
        ..Default::default()
    };

    let response: ApiResponse<Validation> = mailerlite
        .email()
        .validate_settings(email_id, &settings)
        .await?;

    if !response.valid {
        println!("{:#?}", response.errors);
    }

    Ok(())
}
