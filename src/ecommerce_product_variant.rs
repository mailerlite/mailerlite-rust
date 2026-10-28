use reqwest::Method;
use serde_json::Value;

use crate::{ecommerce::send, ecommerce::shop_path, response::Response, MailerLite};

#[derive(Debug)]
pub struct EcommerceProductVariant {
    mailerlite: MailerLite,
}

impl EcommerceProductVariant {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, shop_id: String, product_id: String) -> Response {
        send(&self.mailerlite, Method::GET, shop_path(&shop_id, &format!("/products/{}/variants", product_id)), Vec::new(), None).await
    }

    pub async fn sync(&self, shop_id: String, product_id: String, variants: Value) -> Response {
        send(&self.mailerlite, Method::POST, shop_path(&shop_id, &format!("/products/{}/variants", product_id)), Vec::new(), Some(variants)).await
    }

    pub async fn update(&self, shop_id: String, product_id: String, variant_id: String, body: Value) -> Response {
        send(&self.mailerlite, Method::PUT, shop_path(&shop_id, &format!("/products/{}/variants/{}", product_id, variant_id)), Vec::new(), Some(body)).await
    }

    pub async fn delete(&self, shop_id: String, product_id: String, variant_id: String) -> Response {
        send(&self.mailerlite, Method::DELETE, shop_path(&shop_id, &format!("/products/{}/variants/{}", product_id, variant_id)), Vec::new(), None).await
    }
}
