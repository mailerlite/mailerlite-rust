use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Subscriber ID");

    let shop_id: String = String::from("Your Shop ID");

    let response: Response = mailerlite.subscriber().ecommerce_stats(id, shop_id).await;

    println!("{:#?}", response);
}
