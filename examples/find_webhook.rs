use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Webhook ID");

    let response: Response = mailerlite.webhook().find(id).await;

    println!("{:#?}", response);
}
