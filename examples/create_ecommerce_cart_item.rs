use mailerlite_rs::{response::Response, MailerLite};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let cart_id: String = String::from("Your Cart ID");

    let body: Value = json!({ "ecommerce_product_id": "Your Product ID", "quantity": 1, "price": 19.99 });

    let response: Response = mailerlite.ecommerce_cart_item().create(shop_id, cart_id, body).await;

    println!("{:#?}", response);
}
