use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let cart_id: String = String::from("Your Cart ID");

    let data: Data = Data::new()
        .add("ecommerce_product_id", "Your Product ID")
        .add("quantity", "1")
        .add("price", "19.99");

    let response: Response = mailerlite.ecommerce_cart_item().create(shop_id, cart_id, data.clone()).await;

    println!("{:#?}", response);
}
