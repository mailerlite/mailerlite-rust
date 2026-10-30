use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let automation_id: String = String::from("Your Automation ID");

    let step_id: String = String::from("Your Step ID");

    let response: Response = mailerlite
        .automation_step()
        .ab_stat(automation_id, step_id)
        .await;

    println!("{:#?}", response);
}
