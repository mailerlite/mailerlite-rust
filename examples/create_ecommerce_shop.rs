use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let data: Data = Data::new()
        .add("name", "Dummy Shop")
        .add("url", "https://shop.example.com")
        .add("currency", "EUR");

    let response: Response = mailerlite.ecommerce_shop().create(data.clone()).await;

    println!("{:#?}", response);
}
