use crate::{parameter::Parameter, response::Response, MailerLite, BASE_PATH};
use serde_json::Value;

const END_POINT: &str = "ecommerce/shops";

#[derive(Debug)]
pub struct EcommerceShop {
    mailerlite: MailerLite,
}

impl EcommerceShop {
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

    pub async fn find(&self, shop_id: String) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, shop_id);

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

    pub async fn create(&self, body: Value) -> Response {
        let url: String = format!("{}{}", BASE_PATH, END_POINT);

        Response::new(
            self.mailerlite
                .client
                .request
                .post(url)
                .json(&body)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn update(&self, shop_id: String, body: Value) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, shop_id);

        Response::new(
            self.mailerlite
                .client
                .request
                .put(url)
                .json(&body)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn delete(&self, shop_id: String) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, shop_id);

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

    pub async fn stats(&self, shop_id: String, start_date: String, end_date: String) -> Response {
        let url: String = format!("{}{}/{}/reports/stats", BASE_PATH, END_POINT, shop_id);

        Response::new(
            self.mailerlite
                .client
                .request
                .get(url)
                .query(&[("start_date", start_date), ("end_date", end_date)])
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }
}
