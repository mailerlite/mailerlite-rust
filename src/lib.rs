use automation::Automation;
use batch::Batch;
use campaign::Campaign;
use client::Client;
use field::Field;
use form::Form;
use group::Group;
use segment::Segment;
use subscriber::Subscriber;
use timezone::Timezone;
use webhook::Webhook;

pub mod automation;
pub mod batch;
pub mod campaign;
pub mod client;
pub mod data;
pub mod field;
pub mod form;
pub mod group;
pub mod parameter;
pub mod response;
pub mod segment;
pub mod subscriber;
pub mod timezone;
pub mod webhook;

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

    pub fn field(&self) -> Field {
        Field::new(self.clone())
    }

    pub fn form(&self) -> Form {
        Form::new(self.clone())
    }

    pub fn automation(&self) -> Automation {
        Automation::new(self.clone())
    }

    pub fn webhook(&self) -> Webhook {
        Webhook::new(self.clone())
    }

    pub fn batch(&self) -> Batch {
        Batch::new(self.clone())
    }

    pub fn timezone(&self) -> Timezone {
        Timezone::new(self.clone())
    }
}
