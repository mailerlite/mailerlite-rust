use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Automation ID");

    let parameter: Parameter = Parameter::new().add("filter[status]", "completed");

    let response: Response = mailerlite.automation().subscribers_activity(id, parameter.clone()).await;

    println!("{:#?}", response);
}
