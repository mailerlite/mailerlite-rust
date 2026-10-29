use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let cart_id: String = String::from("Your Cart ID");

    let parameter: Parameter = Parameter::new().add("limit", "25");

    let response: Response = mailerlite
        .ecommerce_cart_item()
        .get(shop_id, cart_id, parameter.clone())
        .await;

    println!("{:#?}", response);
}
