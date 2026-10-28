use mailerlite_rs::{response::Response, MailerLite};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let variant_id: String = String::from("Your Variant ID");

    let body: Value = json!({ "name": "Extra large", "attributes": { "size": "XL" } });

    let response: Response = mailerlite.ecommerce_product_variant().update(shop_id, product_id, variant_id, body).await;

    println!("{:#?}", response);
}
