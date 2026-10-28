use reqwest::Method;

use crate::{ecommerce::send, response::Response, MailerLite};

#[derive(Debug)]
pub struct EcommerceSetting {
    mailerlite: MailerLite,
}

impl EcommerceSetting {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn find(&self) -> Response {
        send(&self.mailerlite, Method::GET, "ecommerce/settings".to_string(), Vec::new(), None).await
    }
}
