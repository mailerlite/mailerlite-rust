use crate::{data::Data, parameter::Parameter, response::Response, MailerLite};

const END_POINT: &str = "groups";

#[derive(Debug)]
pub struct Group {
    mailerlite: MailerLite,
}

impl Group {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, query: Parameter) -> Response {
        let url: String = format!("{}{}", self.mailerlite.base_url, END_POINT);

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

    pub async fn update(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = format!("{}{}/{}", self.mailerlite.base_url, END_POINT, id.as_ref());

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
        let url: String = format!("{}{}/{}", self.mailerlite.base_url, END_POINT, id.as_ref());

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
        let url: String = format!(
            "{}{}/{}/subscribers",
            self.mailerlite.base_url,
            END_POINT,
            id.as_ref()
        );

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

    pub async fn import_subscribers(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = format!(
            "{}{}/{}/import-subscribers",
            self.mailerlite.base_url,
            END_POINT,
            id.as_ref()
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

    pub async fn assign_subscriber(
        &self,
        group_id: impl AsRef<str>,
        subscriber_id: impl AsRef<str>,
    ) -> Response {
        let url: String = format!(
            "{}subscribers/{}/groups/{}",
            self.mailerlite.base_url,
            subscriber_id.as_ref(),
            group_id.as_ref()
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

    pub async fn unassign_subscriber(
        &self,
        group_id: impl AsRef<str>,
        subscriber_id: impl AsRef<str>,
    ) -> Response {
        let url: String = format!(
            "{}subscribers/{}/groups/{}",
            self.mailerlite.base_url,
            subscriber_id.as_ref(),
            group_id.as_ref()
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
