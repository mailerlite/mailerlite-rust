use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let automation_id: String = String::from("Your Automation ID");

    let step_id: String = String::from("Your Step ID");

    let parameter: Parameter = Parameter::new()
        .add("status", "queued")
        .add("filter[search]", "dummy@example.com");

    let response: Response = mailerlite
        .automation_step()
        .subscribers(automation_id, step_id, parameter.clone())
        .await;

    println!("{:#?}", response);
}
