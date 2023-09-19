use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let data: Data = Data::new().add("name", "Dummy Group");

    let response: Response = mailerlite.group().create(data.clone()).await;

    println!("{:#?}", response);
}
