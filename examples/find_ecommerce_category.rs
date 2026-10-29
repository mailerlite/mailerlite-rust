use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let category_id: String = String::from("Your Category ID");

    let response: Response = mailerlite
        .ecommerce_category()
        .find(shop_id, category_id)
        .await;

    println!("{:#?}", response);
}
