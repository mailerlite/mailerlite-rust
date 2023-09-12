use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let group_id: String = String::from("Your Group ID");

    let subscriber_id: String = String::from("Your Subscriber ID");

    let response: Response = mailerlite
        .group()
        .assign_subscriber(group_id, subscriber_id)
        .await;

    println!("{:#?}", response);
}
