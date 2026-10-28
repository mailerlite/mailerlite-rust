use crate::{data::Data, response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "ecommerce/shops";

#[derive(Debug)]
pub struct EcommerceProductVariant {
    mailerlite: MailerLite,
}

impl EcommerceProductVariant {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, shop_id: String, product_id: String) -> Response {
        let url: String = format!("{}{}/{}/products/{}/variants", BASE_PATH, END_POINT, shop_id, product_id);

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

    pub async fn sync(&self, shop_id: String, product_id: String, form: Data) -> Response {
        let url: String = format!("{}{}/{}/products/{}/variants", BASE_PATH, END_POINT, shop_id, product_id);

        Response::new(
            self.mailerlite
                .client
                .request
                .post(url)
                .form(&form.data)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn update(&self, shop_id: String, product_id: String, variant_id: String, form: Data) -> Response {
        let url: String = format!("{}{}/{}/products/{}/variants/{}", BASE_PATH, END_POINT, shop_id, product_id, variant_id);

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

    pub async fn delete(&self, shop_id: String, product_id: String, variant_id: String) -> Response {
        let url: String = format!("{}{}/{}/products/{}/variants/{}", BASE_PATH, END_POINT, shop_id, product_id, variant_id);

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
}
