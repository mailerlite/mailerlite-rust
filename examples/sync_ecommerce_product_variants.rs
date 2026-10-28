use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let shop_id: String = String::from("Your Shop ID");

    let product_id: String = String::from("Your Product ID");

    let data: Data = Data::new()
        .add("0[name]", "Large")
        .add("0[attributes][size]", "L")
        .add("0[resource_id]", "variant-l");

    let response: Response = mailerlite.ecommerce_product_variant().sync(shop_id, product_id, data.clone()).await;

    println!("{:#?}", response);
}
