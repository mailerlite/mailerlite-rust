use mailerlite_rust::{form::Form, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let form: Form = Form::new().add("email", "john@gmail.com");

    let response: Response = mailerlite.subscriber().create(form.clone()).await;

    println!("{:#?}", response);
}
