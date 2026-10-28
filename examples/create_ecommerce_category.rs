use mailerlite_rs::{response::Response, MailerLite};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let body: Value = json!({ "name": "Dummy Category" });

    let response: Response = mailerlite.ecommerce_category().create(shop_id, body).await;

    println!("{:#?}", response);
}
