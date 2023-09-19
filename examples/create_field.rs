use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let data: Data = Data::new().add("name", "Dummy Field").add("type", "text");

    let response: Response = mailerlite.field().create(data.clone()).await;

    println!("{:#?}", response);
}
