use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Subscriber ID");

    let shop_id: String = String::from("Your Shop ID");

    let parameter: Parameter = Parameter::new().add("page", "1");

    let response: Response = mailerlite
        .subscriber()
        .ecommerce_orders(id, shop_id, parameter.clone())
        .await;

    println!("{:#?}", response);
}
