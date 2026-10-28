use reqwest::Method;
use serde_json::Value;

use crate::{ecommerce::send, ecommerce::shop_path, ecommerce::query_of, parameter::Parameter, response::Response, MailerLite};

#[derive(Debug)]
pub struct EcommerceCategory {
    mailerlite: MailerLite,
}

impl EcommerceCategory {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, shop_id: String, query: Parameter) -> Response {
        send(&self.mailerlite, Method::GET, shop_path(&shop_id, "/categories"), query_of(query), None).await
    }

    pub async fn find(&self, shop_id: String, category_id: String) -> Response {
        send(&self.mailerlite, Method::GET, shop_path(&shop_id, &format!("/categories/{}", category_id)), Vec::new(), None).await
    }

    pub async fn create(&self, shop_id: String, body: Value) -> Response {
        send(&self.mailerlite, Method::POST, shop_path(&shop_id, "/categories"), Vec::new(), Some(body)).await
    }

    pub async fn update(&self, shop_id: String, category_id: String, body: Value) -> Response {
        send(&self.mailerlite, Method::PUT, shop_path(&shop_id, &format!("/categories/{}", category_id)), Vec::new(), Some(body)).await
    }

    pub async fn delete(&self, shop_id: String, category_id: String) -> Response {
        send(&self.mailerlite, Method::DELETE, shop_path(&shop_id, &format!("/categories/{}", category_id)), Vec::new(), None).await
    }

    pub async fn products(&self, shop_id: String, category_id: String, query: Parameter) -> Response {
        send(&self.mailerlite, Method::GET, shop_path(&shop_id, &format!("/categories/{}/products", category_id)), query_of(query), None).await
    }

    pub async fn assign_product(&self, shop_id: String, category_id: String, product_id: String) -> Response {
        send(&self.mailerlite, Method::PUT, shop_path(&shop_id, &format!("/categories/{}/products/{}", category_id, product_id)), Vec::new(), None).await
    }

    pub async fn unassign_product(&self, shop_id: String, category_id: String, product_id: String) -> Response {
        send(&self.mailerlite, Method::DELETE, shop_path(&shop_id, &format!("/categories/{}/products/{}", category_id, product_id)), Vec::new(), None).await
    }
}
