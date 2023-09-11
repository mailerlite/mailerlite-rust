use mailerlite_rs::{form::Form, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Campaign ID");

    let form: Form = Form::new().add("delivery", "instant");

    let response: Response = mailerlite.campaign().schedule(id, form.clone()).await;

    println!("{:#?}", response);
}
