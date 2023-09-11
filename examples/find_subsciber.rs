use mailerlite_rust::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Subscriber ID");

    let response: Response = mailerlite.subscriber().find(id).await;

    println!("{:#?}", response);
}
