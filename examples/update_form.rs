use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Form ID");

    let data: Data = Data::new().add("name", "Dummy Form");

    let response: Response = mailerlite.form().update(id, data.clone()).await;

    println!("{:#?}", response);
}
