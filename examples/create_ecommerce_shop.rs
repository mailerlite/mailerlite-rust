use mailerlite_rs::{response::Response, MailerLite};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let body: Value = json!({ "name": "Dummy Shop", "url": "https://shop.example.com", "currency": "EUR" });

    let response: Response = mailerlite.ecommerce_shop().create(body).await;

    println!("{:#?}", response);
}
