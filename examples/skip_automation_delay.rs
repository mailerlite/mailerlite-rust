use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Automation ID");

    let data: Data = Data::new()
        .add("filter[status]", "active")
        .add("filter[next_step]", "Your Step ID");

    let response: Response = mailerlite.automation().skip_delay(id, data.clone()).await;

    println!("{:#?}", response);
}
