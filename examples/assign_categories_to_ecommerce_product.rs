use mailerlite_rs::{response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let category_ids: Vec<String> = vec![String::from("Your Category ID")];

    let response: Response = mailerlite.ecommerce_product().assign_categories(shop_id, product_id, category_ids, false).await;

    println!("{:#?}", response);
}
