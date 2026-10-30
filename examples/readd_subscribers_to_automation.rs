use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Automation ID");

    let data: Data = Data::new()
        .add("readd_subscribers_to", "last_completed_step")
        .add("filter[status]", "completed");

    let response: Response = mailerlite
        .automation()
        .readd_subscribers(id, data.clone())
        .await;

    println!("{:#?}", response);
}
