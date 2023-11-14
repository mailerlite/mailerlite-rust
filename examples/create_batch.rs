use mailerlite_rs::{data::Data, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let data: Data = Data::new()
        .add("requests[0][method]", "POST")
        .add("requests[0][path]", "api/subscribers")
        .add("requests[0][body][email]", "dummy@example.com");

    let response: Response = mailerlite.batch().create(data.clone()).await;

    println!("{:#?}", response);
}
