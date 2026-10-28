use crate::{response::Response, MailerLite, BASE_PATH};
use serde_json::Value;

const END_POINT: &str = "ecommerce/shops";

#[derive(Debug)]
pub struct EcommerceImport {
    mailerlite: MailerLite,
}

impl EcommerceImport {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn products(&self, shop_id: String, body: Value) -> Response {
        let url: String = format!("{}{}/{}/products/import", BASE_PATH, END_POINT, shop_id);

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

    pub async fn categories(&self, shop_id: String, body: Value) -> Response {
        let url: String = format!("{}{}/{}/categories/import", BASE_PATH, END_POINT, shop_id);

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

    pub async fn customers(&self, shop_id: String, body: Value) -> Response {
        let url: String = format!("{}{}/{}/customers/import", BASE_PATH, END_POINT, shop_id);

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

    pub async fn orders(&self, shop_id: String, body: Value) -> Response {
        let url: String = format!("{}{}/{}/orders/import", BASE_PATH, END_POINT, shop_id);

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
}
