use crate::{parameter::Parameter, response::Response, MailerLite, BASE_PATH};
use serde_json::{json, Value};

const END_POINT: &str = "ecommerce/shops";

#[derive(Debug)]
pub struct EcommerceProduct {
    mailerlite: MailerLite,
}

impl EcommerceProduct {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, shop_id: String, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/products", BASE_PATH, END_POINT, shop_id);

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

    pub async fn find(&self, shop_id: String, product_id: String) -> Response {
        let url: String = format!("{}{}/{}/products/{}", BASE_PATH, END_POINT, shop_id, product_id);

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

    pub async fn create(&self, shop_id: String, body: Value) -> Response {
        let url: String = format!("{}{}/{}/products", BASE_PATH, END_POINT, shop_id);

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

    pub async fn update(&self, shop_id: String, product_id: String, body: Value) -> Response {
        let url: String = format!("{}{}/{}/products/{}", BASE_PATH, END_POINT, shop_id, product_id);

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

    pub async fn delete(&self, shop_id: String, product_id: String) -> Response {
        let url: String = format!("{}{}/{}/products/{}", BASE_PATH, END_POINT, shop_id, product_id);

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

    pub async fn change_status(&self, shop_id: String, product_id: String, status: bool) -> Response {
        let url: String = format!("{}{}/{}/products/{}/change-status", BASE_PATH, END_POINT, shop_id, product_id);

        Response::new(
            self.mailerlite
                .client
                .request
                .put(url)
                .json(&json!({ "status": status }))
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn assign_categories(&self, shop_id: String, product_id: String, category_ids: Vec<String>, replace: bool) -> Response {
        let url: String = format!("{}{}/{}/products/{}/categories/multiple", BASE_PATH, END_POINT, shop_id, product_id);

        Response::new(
            self.mailerlite
                .client
                .request
                .put(url)
                .json(&json!({ "categories": category_ids, "replace": replace }))
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }
}
