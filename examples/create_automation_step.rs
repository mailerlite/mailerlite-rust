use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let automation_id: String = String::from("Your Automation ID");

    let data: Data = Data::new()
        .add("type", "delay")
        .add("parent_id", "Your Step ID")
        .add("data[unit]", "days")
        .add("data[value]", "1");

    let response: Response = mailerlite
        .automation_step()
        .create(automation_id, data.clone())
        .await;

    println!("{:#?}", response);
}
