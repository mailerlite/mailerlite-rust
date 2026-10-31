//! Get brand styles. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{MailerLite, Response, models::brand_styles::BrandStyles};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let response: Response<BrandStyles> = mailerlite.brand_styles().get().await?;

    for preset in &response.data.presets {
        println!("{}: {}", preset.identifier, preset.name);
    }

    Ok(())
}
