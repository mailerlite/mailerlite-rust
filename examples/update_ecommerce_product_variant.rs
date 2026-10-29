use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let variant_id: String = String::from("Your Variant ID");

    let data: Data = Data::new()
        .add("name", "Extra large")
        .add("attributes[size]", "XL");

    let response: Response = mailerlite
        .ecommerce_product_variant()
        .update(shop_id, product_id, variant_id, data.clone())
        .await;

    println!("{:#?}", response);
}
