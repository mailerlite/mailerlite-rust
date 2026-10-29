use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let data: Data = Data::new().add("status", "0");

    let response: Response = mailerlite
        .ecommerce_product()
        .change_status(shop_id, product_id, data.clone())
        .await;

    println!("{:#?}", response);
}
