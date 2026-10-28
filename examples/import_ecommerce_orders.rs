use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("orders[0][customer][email]", "dummy@example.com")
        .add("orders[0][cart][items][0][ecommerce_product_id]", "Your Product ID")
        .add("orders[0][cart][items][0][quantity]", "1")
        .add("orders[0][cart][items][0][price]", "19.99")
        .add("orders[0][status]", "complete");

    let response: Response = mailerlite.ecommerce_import().orders(shop_id, data.clone()).await;

    println!("{:#?}", response);
}
