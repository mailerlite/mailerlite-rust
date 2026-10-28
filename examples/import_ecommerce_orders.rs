use mailerlite_rs::{response::Response, MailerLite};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let body: Value = json!([{
        "customer": { "email": "dummy@example.com", "create_subscriber": true },
        "cart": { "items": [{ "ecommerce_product_id": "Your Product ID", "quantity": 1, "price": 19.99 }] },
        "status": "complete"
    }]);

    let response: Response = mailerlite.ecommerce_import().orders(shop_id, body).await;

    println!("{:#?}", response);
}
