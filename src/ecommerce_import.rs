use reqwest::Method;
use serde_json::Value;

use crate::{ecommerce::send, ecommerce::shop_path, response::Response, MailerLite};

#[derive(Debug)]
pub struct EcommerceImport {
    mailerlite: MailerLite,
}

impl EcommerceImport {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn products(&self, shop_id: String, products: Value) -> Response {
        send(&self.mailerlite, Method::POST, shop_path(&shop_id, "/products/import"), Vec::new(), Some(products)).await
    }

    pub async fn categories(&self, shop_id: String, categories: Value) -> Response {
        send(&self.mailerlite, Method::POST, shop_path(&shop_id, "/categories/import"), Vec::new(), Some(categories)).await
    }

    pub async fn customers(&self, shop_id: String, customers: Value) -> Response {
        send(&self.mailerlite, Method::POST, shop_path(&shop_id, "/customers/import"), Vec::new(), Some(customers)).await
    }

    pub async fn orders(&self, shop_id: String, orders: Value) -> Response {
        send(&self.mailerlite, Method::POST, shop_path(&shop_id, "/orders/import"), Vec::new(), Some(serde_json::json!({ "orders": orders }))).await
    }
}
