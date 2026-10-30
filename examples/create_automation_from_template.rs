use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let template_id: String = String::from("Your Automation Template ID");

    let response: Response = mailerlite.automation_template().select(template_id).await;

    println!("{:#?}", response);
}
