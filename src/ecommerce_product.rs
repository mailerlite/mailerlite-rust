use reqwest::Method;
use serde_json::Value;

use crate::{ecommerce::send, ecommerce::shop_path, ecommerce::query_of, parameter::Parameter, response::Response, MailerLite};

#[derive(Debug)]
pub struct EcommerceProduct {
    mailerlite: MailerLite,
}

impl EcommerceProduct {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, shop_id: String, query: Parameter) -> Response {
        send(&self.mailerlite, Method::GET, shop_path(&shop_id, "/products"), query_of(query), None).await
    }

    pub async fn find(&self, shop_id: String, product_id: String) -> Response {
        send(&self.mailerlite, Method::GET, shop_path(&shop_id, &format!("/products/{}", product_id)), Vec::new(), None).await
    }

    pub async fn create(&self, shop_id: String, body: Value) -> Response {
        send(&self.mailerlite, Method::POST, shop_path(&shop_id, "/products"), Vec::new(), Some(body)).await
    }

    pub async fn update(&self, shop_id: String, product_id: String, body: Value) -> Response {
        send(&self.mailerlite, Method::PUT, shop_path(&shop_id, &format!("/products/{}", product_id)), Vec::new(), Some(body)).await
    }

    pub async fn delete(&self, shop_id: String, product_id: String) -> Response {
        send(&self.mailerlite, Method::DELETE, shop_path(&shop_id, &format!("/products/{}", product_id)), Vec::new(), None).await
    }

    pub async fn change_status(&self, shop_id: String, product_id: String, enabled: bool) -> Response {
        send(&self.mailerlite, Method::PUT, shop_path(&shop_id, &format!("/products/{}/change-status", product_id)), Vec::new(), Some(serde_json::json!({ "status": enabled }))).await
    }

    pub async fn assign_categories(&self, shop_id: String, product_id: String, category_ids: Vec<String>, replace: bool) -> Response {
        send(&self.mailerlite, Method::PUT, shop_path(&shop_id, &format!("/products/{}/categories/multiple", product_id)), Vec::new(), Some(serde_json::json!({ "categories": category_ids, "replace": replace }))).await
    }
}
