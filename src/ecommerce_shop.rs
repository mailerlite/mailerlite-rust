use reqwest::Method;
use serde_json::Value;

use crate::{ecommerce::send, ecommerce::shop_path, ecommerce::query_of, parameter::Parameter, response::Response, MailerLite};

#[derive(Debug)]
pub struct EcommerceShop {
    mailerlite: MailerLite,
}

impl EcommerceShop {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, query: Parameter) -> Response {
        send(&self.mailerlite, Method::GET, "ecommerce/shops".to_string(), query_of(query), None).await
    }

    pub async fn find(&self, shop_id: String) -> Response {
        send(&self.mailerlite, Method::GET, shop_path(&shop_id, ""), Vec::new(), None).await
    }

    pub async fn create(&self, body: Value) -> Response {
        send(&self.mailerlite, Method::POST, "ecommerce/shops".to_string(), Vec::new(), Some(body)).await
    }

    pub async fn update(&self, shop_id: String, body: Value) -> Response {
        send(&self.mailerlite, Method::PUT, shop_path(&shop_id, ""), Vec::new(), Some(body)).await
    }

    pub async fn delete(&self, shop_id: String) -> Response {
        send(&self.mailerlite, Method::DELETE, shop_path(&shop_id, ""), Vec::new(), None).await
    }

    pub async fn stats(&self, shop_id: String, start_date: String, end_date: String) -> Response {
        send(&self.mailerlite, Method::GET, shop_path(&shop_id, "/reports/stats"), vec![("start_date", start_date), ("end_date", end_date)], None).await
    }
}
