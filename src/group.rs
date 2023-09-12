use crate::{form::Form, parameter::Parameter, response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "groups";

#[derive(Debug)]
pub struct Group {
    mailerlite: MailerLite,
}

impl Group {
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

    pub async fn update(&self, id: String, form: Form) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, id);

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

    pub async fn delete(&self, id: String) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, id);

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

    pub async fn get_subscribers(&self, id: String, data: Parameter) -> Response {
        let url: String = format!("{}{}/{}/subscribers", BASE_PATH, END_POINT, id);

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

    pub async fn assign_subscriber(&self, group_id: String, subscriber_id: String) -> Response {
        let url: String = format!(
            "{}subscribers/{}/groups/{}",
            BASE_PATH, subscriber_id, group_id
        );

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

    pub async fn unassign_subscriber(&self, group_id: String, subscriber_id: String) -> Response {
        let url: String = format!(
            "{}subscribers/{}/groups/{}",
            BASE_PATH, subscriber_id, group_id
        );

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
}
