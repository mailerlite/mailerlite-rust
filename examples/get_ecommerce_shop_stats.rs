use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let start_date: String = String::from("2026-10-01");

    let end_date: String = String::from("2026-10-31");

    let response: Response = mailerlite.ecommerce_shop().stats(shop_id, start_date, end_date).await;

    println!("{:#?}", response);
}
