use crate::{data::Data, response::Response, MailerLite};

const END_POINT: &str = "batch";

#[derive(Debug)]
pub struct Batch {
    mailerlite: MailerLite,
}

impl Batch {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn create(&self, form: Data) -> Response {
        let url: String = format!("{}{}", self.mailerlite.base_url, END_POINT);

        Response::new(
            self.mailerlite
                .client
                .request
                .post(url)
                .form(&form.data)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }
}
