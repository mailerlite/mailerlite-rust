use crate::{response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "timezones";

#[derive(Debug)]
pub struct Timezone {
    mailerlite: MailerLite,
}

impl Timezone {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self) -> Response {
        let url: String = format!("{}{}", BASE_PATH, END_POINT);

        Response::new(
            self.mailerlite
                .client
                .request
                .get(url)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }
}
