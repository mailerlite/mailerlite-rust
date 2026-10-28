use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let import_id: String = String::from("Your Import ID");

    let response: Response = mailerlite.subscriber().import_status(import_id).await;

    println!("{:#?}", response);
}
