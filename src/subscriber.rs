use crate::{data::Data, parameter::Parameter, response::Response, MailerLite, BASE_PATH};
use crate::ecommerce::{query_of, send};
use reqwest::Method;

const END_POINT: &str = "subscribers";

#[derive(Debug)]
pub struct Subscriber {
    mailerlite: MailerLite,
}

impl Subscriber {
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

    pub async fn find(&self, id: String) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, id);

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

    pub async fn update(&self, id: String, form: Data) -> Response {
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

    pub async fn ecommerce_stats(&self, id: String, shop_id: String) -> Response {
        send(
            &self.mailerlite,
            Method::GET,
            format!("{}/{}/reports/ecommerce", END_POINT, id),
            vec![("shop_id", shop_id)],
            None,
        )
        .await
    }

    pub async fn ecommerce_orders(&self, id: String, shop_id: String, query: Parameter) -> Response {
        let mut query: Vec<(&'static str, String)> = query_of(query);
        query.push(("shop_id", shop_id));

        send(
            &self.mailerlite,
            Method::GET,
            format!("{}/{}/reports/ecommerce/orders", END_POINT, id),
            query,
            None,
        )
        .await
    }
}
