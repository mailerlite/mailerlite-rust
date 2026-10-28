use mailerlite_rs::{response::Response, MailerLite};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let body: Value = json!({ "price": 24.99 });

    let response: Response = mailerlite.ecommerce_product().update(shop_id, product_id, body).await;

    println!("{:#?}", response);
}
