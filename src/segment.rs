use crate::{data::Data, parameter::Parameter, response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "segments";

#[derive(Debug)]
pub struct Segment {
    mailerlite: MailerLite,
}

impl Segment {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, query: Parameter) -> Response {
        let url: String = format!("{}{}", BASE_PATH, END_POINT);

        Response::new(
            self.mailerlite
                .client
                .request
                .get(url)
                .query(&query.data)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn update(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, id.as_ref());

        Response::new(
            self.mailerlite
                .client
                .request
                .put(url)
                .form(&form.data)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn delete(&self, id: impl AsRef<str>) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, id.as_ref());

        Response::new(
            self.mailerlite
                .client
                .request
                .delete(url)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn get_subscribers(&self, id: impl AsRef<str>, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/subscribers", BASE_PATH, END_POINT, id.as_ref());

        Response::new(
            self.mailerlite
                .client
                .request
                .get(url)
                .query(&query.data)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }
}
