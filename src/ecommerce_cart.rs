use crate::{data::Data, parameter::Parameter, response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "ecommerce/shops";

#[derive(Debug)]
pub struct EcommerceCart {
    mailerlite: MailerLite,
}

impl EcommerceCart {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, shop_id: String, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/carts", BASE_PATH, END_POINT, shop_id);

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

    pub async fn find(&self, shop_id: String, cart_id: String) -> Response {
        let url: String = format!("{}{}/{}/carts/{}", BASE_PATH, END_POINT, shop_id, cart_id);

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

    pub async fn update(&self, shop_id: String, cart_id: String, form: Data) -> Response {
        let url: String = format!("{}{}/{}/carts/{}", BASE_PATH, END_POINT, shop_id, cart_id);

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
}
