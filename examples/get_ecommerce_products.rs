use mailerlite_rs::{parameter::Parameter, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let parameter: Parameter = Parameter::new().add("filter[enabled]", "true");

    let response: Response = mailerlite
        .ecommerce_product()
        .get(shop_id, parameter.clone())
        .await;

    println!("{:#?}", response);
}
