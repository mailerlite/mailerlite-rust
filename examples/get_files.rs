//! Get files. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{
    MailerLite, Response,
    models::files::{FileSort, FileType, MediaFile},
    pagination::NumberedPage,
    resources::files::ListFilesOptions,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let options: ListFilesOptions = ListFilesOptions {
        name: Some(String::from("summer sale")),
        file_type: Some(FileType::Image),
        sort: Some(FileSort::SizeDesc),
        page: Some(1),
        limit: Some(25),
    };

    let response: Response<NumberedPage<MediaFile>> =
        mailerlite.files().list().options(options).send().await?;

    for file in &response.data.data {
        println!("{}: {}", file.name, file.url);
    }

    Ok(())
}
