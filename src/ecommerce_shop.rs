use crate::{data::Data, parameter::Parameter, response::Response, MailerLite};

const END_POINT: &str = "ecommerce/shops";

#[derive(Debug)]
pub struct EcommerceShop {
    mailerlite: MailerLite,
}

impl EcommerceShop {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, query: Parameter) -> Response {
        let url: String = format!("{}{}", self.mailerlite.base_url, END_POINT);

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

    pub async fn find(&self, shop_id: impl AsRef<str>) -> Response {
        let url: String = format!(
            "{}{}/{}",
            self.mailerlite.base_url,
            END_POINT,
            shop_id.as_ref()
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

    pub async fn create(&self, form: Data) -> Response {
        let url: String = format!("{}{}", self.mailerlite.base_url, END_POINT);

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

    pub async fn update(&self, shop_id: impl AsRef<str>, form: Data) -> Response {
        let url: String = format!(
            "{}{}/{}",
            self.mailerlite.base_url,
            END_POINT,
            shop_id.as_ref()
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

    pub async fn delete(&self, shop_id: impl AsRef<str>) -> Response {
        let url: String = format!(
            "{}{}/{}",
            self.mailerlite.base_url,
            END_POINT,
            shop_id.as_ref()
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

    pub async fn stats(
        &self,
        shop_id: impl AsRef<str>,
        start_date: impl AsRef<str>,
        end_date: impl AsRef<str>,
    ) -> Response {
        let url: String = format!(
            "{}{}/{}/reports/stats",
            self.mailerlite.base_url,
            END_POINT,
            shop_id.as_ref()
        );

        Response::new(
            self.mailerlite
                .client
                .request
                .get(url)
                .query(&[
                    ("start_date", start_date.as_ref()),
                    ("end_date", end_date.as_ref()),
                ])
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }
}
