use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("0[name]", "Dummy Product")
        .add("0[price]", "19.99")
        .add("0[url]", "https://shop.example.com/dummy-product");

    let response: Response = mailerlite
        .ecommerce_import()
        .products(shop_id, data.clone())
        .await;

    println!("{:#?}", response);
}
