use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Group ID");

    let parameter: Parameter = Parameter::new().add("filter[status]", "unsubscribed");

    let response: Response = mailerlite
        .group()
        .get_subscribers(id, parameter.clone())
        .await;

    println!("{:#?}", response);
}
