use crate::{data::Data, response::Response, MailerLite};

const END_POINT: &str = "ecommerce/shops";

#[derive(Debug)]
pub struct EcommerceImport {
    mailerlite: MailerLite,
}

impl EcommerceImport {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn products(&self, shop_id: impl AsRef<str>, form: Data) -> Response {
        let url: String = format!(
            "{}{}/{}/products/import",
            self.mailerlite.base_url,
            END_POINT,
            shop_id.as_ref()
        );

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

    pub async fn categories(&self, shop_id: impl AsRef<str>, form: Data) -> Response {
        let url: String = format!(
            "{}{}/{}/categories/import",
            self.mailerlite.base_url,
            END_POINT,
            shop_id.as_ref()
        );

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

    pub async fn customers(&self, shop_id: impl AsRef<str>, form: Data) -> Response {
        let url: String = format!(
            "{}{}/{}/customers/import",
            self.mailerlite.base_url,
            END_POINT,
            shop_id.as_ref()
        );

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

    pub async fn orders(&self, shop_id: impl AsRef<str>, form: Data) -> Response {
        let url: String = format!(
            "{}{}/{}/orders/import",
            self.mailerlite.base_url,
            END_POINT,
            shop_id.as_ref()
        );

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
}
