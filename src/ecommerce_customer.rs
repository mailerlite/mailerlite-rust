use crate::{data::Data, parameter::Parameter, response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "ecommerce/shops";

#[derive(Debug)]
pub struct EcommerceCustomer {
    mailerlite: MailerLite,
}

impl EcommerceCustomer {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, shop_id: impl AsRef<str>, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/customers", BASE_PATH, END_POINT, shop_id.as_ref());

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

    pub async fn find(&self, shop_id: impl AsRef<str>, customer_id: impl AsRef<str>) -> Response {
        let url: String = format!(
            "{}{}/{}/customers/{}",
            BASE_PATH,
            END_POINT,
            shop_id.as_ref(),
            customer_id.as_ref()
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
        let url: String = format!("{}{}/{}/customers", BASE_PATH, END_POINT, shop_id.as_ref());

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
        customer_id: impl AsRef<str>,
        form: Data,
    ) -> Response {
        let url: String = format!(
            "{}{}/{}/customers/{}",
            BASE_PATH,
            END_POINT,
            shop_id.as_ref(),
            customer_id.as_ref()
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

    pub async fn delete(&self, shop_id: impl AsRef<str>, customer_id: impl AsRef<str>) -> Response {
        let url: String = format!(
            "{}{}/{}/customers/{}",
            BASE_PATH,
            END_POINT,
            shop_id.as_ref(),
            customer_id.as_ref()
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
}
