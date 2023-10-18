use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let parameter: Parameter = Parameter::new().add("filter[type]", "opened");

    let response: Response = mailerlite.campaign().subscribers_activity(id, parameter.clone()).await;

    println!("{:#?}", response);
}
