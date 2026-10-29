use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("name", "Dummy Product")
        .add("price", "19.99")
        .add("url", "https://shop.example.com/dummy-product");

    let response: Response = mailerlite
        .ecommerce_product()
        .create(shop_id, data.clone())
        .await;

    println!("{:#?}", response);
}
