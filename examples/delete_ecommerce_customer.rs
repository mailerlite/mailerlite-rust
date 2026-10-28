use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let customer_id: String = String::from("Your Customer ID");

    let response: Response = mailerlite.ecommerce_customer().delete(shop_id, customer_id).await;

    println!("{:#?}", response);
}
