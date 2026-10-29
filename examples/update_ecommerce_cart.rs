use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let cart_id: String = String::from("Your Cart ID");

    let data: Data = Data::new().add("checkout_url", "https://shop.example.com/cart");

    let response: Response = mailerlite
        .ecommerce_cart()
        .update(shop_id, cart_id, data.clone())
        .await;

    println!("{:#?}", response);
}
