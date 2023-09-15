use mailerlite_rs::{form::Form, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Segment ID");

    let form: Form = Form::new().add("name", "Dummy Segment");

    let response: Response = mailerlite.segment().update(id, form.clone()).await;

    println!("{:#?}", response);
}
