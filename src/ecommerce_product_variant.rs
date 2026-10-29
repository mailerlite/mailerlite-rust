use crate::{data::Data, response::Response, MailerLite};

const END_POINT: &str = "ecommerce/shops";

#[derive(Debug)]
pub struct EcommerceProductVariant {
    mailerlite: MailerLite,
}

impl EcommerceProductVariant {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, shop_id: impl AsRef<str>, product_id: impl AsRef<str>) -> Response {
        let url: String = format!(
            "{}{}/{}/products/{}/variants",
            self.mailerlite.base_url,
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

    pub async fn sync(
        &self,
        shop_id: impl AsRef<str>,
        product_id: impl AsRef<str>,
        form: Data,
    ) -> Response {
        let url: String = format!(
            "{}{}/{}/products/{}/variants",
            self.mailerlite.base_url,
            END_POINT,
            shop_id.as_ref(),
            product_id.as_ref()
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

    pub async fn update(
        &self,
        shop_id: impl AsRef<str>,
        product_id: impl AsRef<str>,
        variant_id: impl AsRef<str>,
        form: Data,
    ) -> Response {
        let url: String = format!(
            "{}{}/{}/products/{}/variants/{}",
            self.mailerlite.base_url,
            END_POINT,
            shop_id.as_ref(),
            product_id.as_ref(),
            variant_id.as_ref()
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

    pub async fn delete(
        &self,
        shop_id: impl AsRef<str>,
        product_id: impl AsRef<str>,
        variant_id: impl AsRef<str>,
    ) -> Response {
        let url: String = format!(
            "{}{}/{}/products/{}/variants/{}",
            self.mailerlite.base_url,
            END_POINT,
            shop_id.as_ref(),
            product_id.as_ref(),
            variant_id.as_ref()
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
