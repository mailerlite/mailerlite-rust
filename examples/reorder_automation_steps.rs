use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let automation_id: String = String::from("Your Automation ID");

    let data: Data = Data::new()
        .add("dragged_step_id", "Your Step ID")
        .add("new_parent_step_id", "Your Parent Step ID");

    let response: Response = mailerlite
        .automation_step()
        .reorder(automation_id, data.clone())
        .await;

    println!("{:#?}", response);
}
