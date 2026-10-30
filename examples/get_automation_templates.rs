use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let parameter: Parameter = Parameter::new()
        .add("filter[type]", "welcome")
        .add("limit", "10");

    let response: Response = mailerlite
        .automation_template()
        .get(parameter.clone())
        .await;

    println!("{:#?}", response);
}
