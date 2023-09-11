use mailerlite_rust::{form::Form, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let form: Form = Form::new()
        .add("name", "Regular Campaign")
        .add("emails[0][subject]", "Test Subject")
        .add("emails[0][from_name]", "John Doe")
        .add("emails[0][from]", "john@gmail.com");

    let response: Response = mailerlite.campaign().update(id, form.clone()).await;

    println!("{:#?}", response);
}
