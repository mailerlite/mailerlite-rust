use crate::{data::Data, parameter::Parameter, response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "campaigns";

#[derive(Debug)]
pub struct Campaign {
    mailerlite: MailerLite,
}

impl Campaign {
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

    pub async fn find(&self, id: impl AsRef<str>) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, id.as_ref());

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

    pub async fn create(&self, form: Data) -> Response {
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

    pub async fn schedule(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = format!("{}{}/{}/schedule", BASE_PATH, END_POINT, id.as_ref());

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

    pub async fn cancel(&self, id: impl AsRef<str>) -> Response {
        let url: String = format!("{}{}/{}/cancel", BASE_PATH, END_POINT, id.as_ref());

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

    pub async fn subscribers_activity(&self, id: impl AsRef<str>, query: Parameter) -> Response {
        let url: String = format!(
            "{}{}/{}/reports/subscriber-activity",
            BASE_PATH,
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

    pub async fn languages(&self) -> Response {
        let url: String = format!("{}{}/languages", BASE_PATH, END_POINT);

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

    pub async fn ecommerce_stats(&self, id: impl AsRef<str>, shop_id: impl AsRef<str>) -> Response {
        let url: String = format!(
            "{}{}/{}/reports/ecommerce",
            BASE_PATH,
            END_POINT,
            id.as_ref()
        );

        Response::new(
            self.mailerlite
                .client
                .request
                .get(url)
                .query(&[("shop_id", shop_id.as_ref())])
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn ecommerce_orders(
        &self,
        id: impl AsRef<str>,
        shop_id: impl AsRef<str>,
        query: Parameter,
    ) -> Response {
        let url: String = format!(
            "{}{}/{}/reports/ecommerce/orders",
            BASE_PATH,
            END_POINT,
            id.as_ref()
        );
        let query: Vec<(&str, &str)> = query
            .data
            .into_iter()
            .filter(|(key, _)| *key != "shop_id")
            .collect();

        Response::new(
            self.mailerlite
                .client
                .request
                .get(url)
                .query(&[("shop_id", shop_id.as_ref())])
                .query(&query)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }
}
