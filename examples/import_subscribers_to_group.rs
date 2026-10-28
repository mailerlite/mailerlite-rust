use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Group ID");

    let data: Data = Data::new()
        .add("subscribers[0][email]", "dummy@example.com")
        .add("subscribers[0][fields][name]", "Dummy");

    let response: Response = mailerlite.group().import_subscribers(id, data.clone()).await;

    println!("{:#?}", response);
}
