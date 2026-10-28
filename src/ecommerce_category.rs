use crate::{data::Data, parameter::Parameter, response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "ecommerce/shops";

#[derive(Debug)]
pub struct EcommerceCategory {
    mailerlite: MailerLite,
}

impl EcommerceCategory {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, shop_id: String, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/categories", BASE_PATH, END_POINT, shop_id);

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

    pub async fn find(&self, shop_id: String, category_id: String) -> Response {
        let url: String = format!("{}{}/{}/categories/{}", BASE_PATH, END_POINT, shop_id, category_id);

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

    pub async fn create(&self, shop_id: String, form: Data) -> Response {
        let url: String = format!("{}{}/{}/categories", BASE_PATH, END_POINT, shop_id);

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

    pub async fn update(&self, shop_id: String, category_id: String, form: Data) -> Response {
        let url: String = format!("{}{}/{}/categories/{}", BASE_PATH, END_POINT, shop_id, category_id);

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

    pub async fn delete(&self, shop_id: String, category_id: String) -> Response {
        let url: String = format!("{}{}/{}/categories/{}", BASE_PATH, END_POINT, shop_id, category_id);

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

    pub async fn products(&self, shop_id: String, category_id: String, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/categories/{}/products", BASE_PATH, END_POINT, shop_id, category_id);

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

    pub async fn assign_product(&self, shop_id: String, category_id: String, product_id: String) -> Response {
        let url: String = format!("{}{}/{}/categories/{}/products/{}", BASE_PATH, END_POINT, shop_id, category_id, product_id);

        Response::new(
            self.mailerlite
                .client
                .request
                .put(url)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn unassign_product(&self, shop_id: String, category_id: String, product_id: String) -> Response {
        let url: String = format!("{}{}/{}/categories/{}/products/{}", BASE_PATH, END_POINT, shop_id, category_id, product_id);

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
