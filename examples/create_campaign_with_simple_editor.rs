use mailerlite_rs::{campaign::EDITOR_SIMPLE, data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let data: Data = Data::new()
        .add("name", "Simple Editor Campaign")
        .add("type", "regular")
        .add("emails[0][subject]", "Test Subject")
        .add("emails[0][from_name]", "John Doe")
        .add("emails[0][from]", "john@gmail.com")
        .add("emails[0][editor]", EDITOR_SIMPLE);

    let response: Response = mailerlite.campaign().create(data.clone()).await;

    println!("Email ID: {}", response.content["data"]["emails"][0]["id"]);
}
