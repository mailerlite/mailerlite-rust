use client::Client;
use subscriber::Subscriber;

pub mod client;
pub mod form;
pub mod parameter;
pub mod response;
pub mod subscriber;

const BASE_PATH: &str = "https://dashboard.mailerlite.dev/api/";

#[derive(Debug, Clone)]
pub struct MailerLite {
    client: Client,
}

impl MailerLite {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(api_key),
        }
    }

    pub fn subscriber(&self) -> Subscriber {
        Subscriber::new(self.clone())
    }
}
