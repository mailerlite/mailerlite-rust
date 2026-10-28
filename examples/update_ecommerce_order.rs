use mailerlite_rs::{response::Response, MailerLite};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let order_id: String = String::from("Your Order ID");

    let body: Value = json!({ "status": "complete" });

    let response: Response = mailerlite.ecommerce_order().update(shop_id, order_id, body).await;

    println!("{:#?}", response);
}
