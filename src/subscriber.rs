use crate::MailerLite;

const END_POINT: &str = "subscribers";

#[derive(Debug)]
pub struct Subscriber {
    mailerlite: MailerLite,
}

impl Subscriber {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }
}
