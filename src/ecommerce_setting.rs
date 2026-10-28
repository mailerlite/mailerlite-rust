use crate::{response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "ecommerce/settings";

#[derive(Debug)]
pub struct EcommerceSetting {
    mailerlite: MailerLite,
}

impl EcommerceSetting {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn find(&self) -> Response {
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
