use client::Client;

pub mod client;
pub mod response;

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
}
