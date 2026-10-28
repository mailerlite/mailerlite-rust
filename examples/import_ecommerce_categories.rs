use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let data: Data = Data::new()
        .add("0[name]", "Dummy Category")
        .add("0[exclude_from_automations]", "0");

    let response: Response = mailerlite.ecommerce_import().categories(shop_id, data.clone()).await;

    println!("{:#?}", response);
}
