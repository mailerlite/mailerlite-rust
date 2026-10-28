use reqwest::Method;
use serde_json::Value;

use crate::{ecommerce::send, ecommerce::shop_path, ecommerce::query_of, parameter::Parameter, response::Response, MailerLite};

#[derive(Debug)]
pub struct EcommerceCart {
    mailerlite: MailerLite,
}

impl EcommerceCart {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, shop_id: String, query: Parameter) -> Response {
        send(&self.mailerlite, Method::GET, shop_path(&shop_id, "/carts"), query_of(query), None).await
    }

    pub async fn find(&self, shop_id: String, cart_id: String) -> Response {
        send(&self.mailerlite, Method::GET, shop_path(&shop_id, &format!("/carts/{}", cart_id)), Vec::new(), None).await
    }

    pub async fn update(&self, shop_id: String, cart_id: String, body: Value) -> Response {
        send(&self.mailerlite, Method::PUT, shop_path(&shop_id, &format!("/carts/{}", cart_id)), Vec::new(), Some(body)).await
    }
}
