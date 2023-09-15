use campaign::Campaign;
use client::Client;
use group::Group;
use segment::Segment;
use subscriber::Subscriber;

pub mod campaign;
pub mod client;
pub mod form;
pub mod group;
pub mod parameter;
pub mod response;
pub mod segment;
pub mod subscriber;

const BASE_PATH: &str = "https://connect.mailerlite.com/api/";

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

    pub fn campaign(&self) -> Campaign {
        Campaign::new(self.clone())
    }

    pub fn group(&self) -> Group {
        Group::new(self.clone())
    }

    pub fn segment(&self) -> Segment {
        Segment::new(self.clone())
    }
}
