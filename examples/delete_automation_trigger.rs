use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let automation_id: String = String::from("Your Automation ID");

    let trigger_id: String = String::from("Your Trigger ID");

    let response: Response = mailerlite
        .automation_trigger()
        .delete(automation_id, trigger_id)
        .await;

    println!("{:#?}", response);
}
