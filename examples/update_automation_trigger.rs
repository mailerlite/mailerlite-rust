use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let automation_id: String = String::from("Your Automation ID");

    let trigger_id: String = String::from("Your Trigger ID");

    let data: Data = Data::new().add("data[group_ids][0]", "Your Group ID");

    let response: Response = mailerlite
        .automation_trigger()
        .update(automation_id, trigger_id, data.clone())
        .await;

    println!("{:#?}", response);
}
