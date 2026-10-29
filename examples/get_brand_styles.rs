use mailerlite_rs::{
    api_response::{ApiResponse, DataOnly},
    brand_style::BrandStyles,
    MailerLite,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let response: ApiResponse<DataOnly<BrandStyles>> = mailerlite.brand_style().get().await?;

    for preset in &response.data.presets {
        println!("{}: {}", preset.identifier, preset.name);
    }

    Ok(())
}
