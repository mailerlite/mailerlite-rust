use crate::{form::Form, parameter::Parameter, response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "campaigns";

#[derive(Debug)]
pub struct Campaign {
    mailerlite: MailerLite,
}

impl Campaign {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, data: Parameter) -> Response {
        let url: String = format!("{}{}", BASE_PATH, END_POINT);

        Response::new(
            self.mailerlite
                .client
                .request
                .get(url)
                .query(&data.data)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn find(&self, data: Parameter) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, data.data[0].1);

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

    pub async fn create(&self, form: Form) -> Response {
        let url: String = format!("{}{}", BASE_PATH, END_POINT);

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

    pub async fn update(&self, parameter: Parameter, form: Form) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, parameter.data[0].1);

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

    pub async fn delete(&self, parameter: Parameter) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, parameter.data[0].1);

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

    pub async fn schedule(&self, parameter: Parameter, form: Form) -> Response {
        let url: String = format!(
            "{}{}/{}/schedule",
            BASE_PATH, END_POINT, parameter.data[0].1
        );

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

    pub async fn cancel(&self, parameter: Parameter) -> Response {
        let url: String = format!("{}{}/{}/cancel", BASE_PATH, END_POINT, parameter.data[0].1);

        Response::new(
            self.mailerlite
                .client
                .request
                .post(url)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }
}
