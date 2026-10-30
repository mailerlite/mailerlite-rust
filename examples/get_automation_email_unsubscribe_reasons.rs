use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Automation ID");

    let email_id: String = String::from("Your Email ID");

    let response: Response = mailerlite
        .automation()
        .email_unsubscribe_reasons(id, email_id)
        .await;

    println!("{:#?}", response);
}
