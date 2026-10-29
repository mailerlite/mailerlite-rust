use crate::{response::Response, MailerLite};

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
        let url: String = format!("{}{}", self.mailerlite.base_url, END_POINT);

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
