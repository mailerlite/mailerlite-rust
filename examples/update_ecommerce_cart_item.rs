use mailerlite_rs::{response::Response, MailerLite};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let cart_id: String = String::from("Your Cart ID");

    let item_id: String = String::from("Your Item ID");

    let body: Value = json!({ "quantity": 2 });

    let response: Response = mailerlite.ecommerce_cart_item().update(shop_id, cart_id, item_id, body).await;

    println!("{:#?}", response);
}
