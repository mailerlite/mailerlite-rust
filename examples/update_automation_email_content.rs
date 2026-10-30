use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Automation ID");

    let email_id: String = String::from("Your Email ID");

    let data: Data = Data::new().add("html", "<p>Hello</p>");

    let response: Response = mailerlite
        .automation()
        .update_email_content(id, email_id, data.clone())
        .await;

    println!("{:#?}", response);
}
