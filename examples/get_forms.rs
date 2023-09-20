use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let form_type: String = String::from("Your Form Type");

    let parameter: Parameter = Parameter::new().add("sort", "created_at");

    let response: Response = mailerlite.form().get(form_type, parameter.clone()).await;

    println!("{:#?}", response);
}
