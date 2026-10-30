use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Automation ID");

    let data: Data = Data::new()
        .add("emails[0]", "dummy@example.com")
        .add("groups[0]", "Your Group ID")
        .add("step_id", "Your Step ID");

    let response: Response = mailerlite
        .automation()
        .add_subscribers(id, data.clone())
        .await;

    println!("{:#?}", response);
}
