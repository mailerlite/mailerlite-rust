use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let order_id: String = String::from("Your Order ID");

    let data: Data = Data::new()
        .add("status", "complete");

    let response: Response = mailerlite.ecommerce_order().update(shop_id, order_id, data.clone()).await;

    println!("{:#?}", response);
}
