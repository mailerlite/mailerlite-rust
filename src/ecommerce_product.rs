use crate::{data::Data, parameter::Parameter, response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "ecommerce/shops";

#[derive(Debug)]
pub struct EcommerceProduct {
    mailerlite: MailerLite,
}

impl EcommerceProduct {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, shop_id: impl AsRef<str>, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/products", BASE_PATH, END_POINT, shop_id.as_ref());

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

    pub async fn find(&self, shop_id: impl AsRef<str>, product_id: impl AsRef<str>) -> Response {
        let url: String = format!(
            "{}{}/{}/products/{}",
            BASE_PATH,
            END_POINT,
            shop_id.as_ref(),
            product_id.as_ref()
        );

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

    pub async fn create(&self, shop_id: impl AsRef<str>, form: Data) -> Response {
        let url: String = format!("{}{}/{}/products", BASE_PATH, END_POINT, shop_id.as_ref());

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

    pub async fn update(
        &self,
        shop_id: impl AsRef<str>,
        product_id: impl AsRef<str>,
        form: Data,
    ) -> Response {
        let url: String = format!(
            "{}{}/{}/products/{}",
            BASE_PATH,
            END_POINT,
            shop_id.as_ref(),
            product_id.as_ref()
        );

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

    pub async fn delete(&self, shop_id: impl AsRef<str>, product_id: impl AsRef<str>) -> Response {
        let url: String = format!(
            "{}{}/{}/products/{}",
            BASE_PATH,
            END_POINT,
            shop_id.as_ref(),
            product_id.as_ref()
        );

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

    pub async fn change_status(
        &self,
        shop_id: impl AsRef<str>,
        product_id: impl AsRef<str>,
        form: Data,
    ) -> Response {
        let url: String = format!(
            "{}{}/{}/products/{}/change-status",
            BASE_PATH,
            END_POINT,
            shop_id.as_ref(),
            product_id.as_ref()
        );

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

    pub async fn assign_categories(
        &self,
        shop_id: impl AsRef<str>,
        product_id: impl AsRef<str>,
        form: Data,
    ) -> Response {
        let url: String = format!(
            "{}{}/{}/products/{}/categories/multiple",
            BASE_PATH,
            END_POINT,
            shop_id.as_ref(),
            product_id.as_ref()
        );

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
