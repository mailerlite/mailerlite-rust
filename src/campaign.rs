use crate::{data::Data, parameter::Parameter, response::Response, MailerLite};

const END_POINT: &str = "campaigns";

/// The `editor` value that builds the emails of a new campaign with the simple editor.
/// Edit these emails with `MailerLite::email`.
pub const EDITOR_SIMPLE: &str = "simple";

#[derive(Debug)]
pub struct Campaign {
    mailerlite: MailerLite,
}

impl Campaign {
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

    pub async fn find(&self, id: impl AsRef<str>) -> Response {
        let url: String = format!("{}{}/{}", self.mailerlite.base_url, END_POINT, id.as_ref());

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

    /// To build an email with the simple editor, add these keys for each email, for example
    /// `emails[0][editor]`:
    ///
    /// - `editor`: the only value is `simple` ([`EDITOR_SIMPLE`]). Use the same value for every email.
    ///   With `editor`, the API rejects `emails.*.content`.
    /// - `template_id`: the template that the new email starts from. Send it only with `editor`. It must
    ///   be a template from the shared gallery. Use the same value for every email.
    ///
    /// Only `create` accepts `editor` and `template_id`. The ID of the new email is in `data.emails[0].id`.
    ///
    /// `Data` accepts only `&'static str` values. If you get the template ID at run time, use
    /// `Box::leak(template_id.into_boxed_str())`. The string then stays in memory until the program stops.
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

    /// Only `create` accepts `editor` and `template_id`, so do not send them here.
    pub async fn update(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = format!("{}{}/{}", self.mailerlite.base_url, END_POINT, id.as_ref());

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

    pub async fn delete(&self, id: impl AsRef<str>) -> Response {
        let url: String = format!("{}{}/{}", self.mailerlite.base_url, END_POINT, id.as_ref());

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

    pub async fn schedule(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = format!(
            "{}{}/{}/schedule",
            self.mailerlite.base_url,
            END_POINT,
            id.as_ref()
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

    pub async fn cancel(&self, id: impl AsRef<str>) -> Response {
        let url: String = format!(
            "{}{}/{}/cancel",
            self.mailerlite.base_url,
            END_POINT,
            id.as_ref()
        );

        Response::new(
            self.mailerlite
                .client
                .request
                .post(url)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn subscribers_activity(&self, id: impl AsRef<str>, query: Parameter) -> Response {
        let url: String = format!(
            "{}{}/{}/reports/subscriber-activity",
            self.mailerlite.base_url,
            END_POINT,
            id.as_ref()
        );

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

    pub async fn languages(&self) -> Response {
        let url: String = format!("{}{}/languages", self.mailerlite.base_url, END_POINT);

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

    pub async fn ecommerce_stats(&self, id: impl AsRef<str>, shop_id: impl AsRef<str>) -> Response {
        let url: String = format!(
            "{}{}/{}/reports/ecommerce",
            self.mailerlite.base_url,
            END_POINT,
            id.as_ref()
        );

        Response::new(
            self.mailerlite
                .client
                .request
                .get(url)
                .query(&[("shop_id", shop_id.as_ref())])
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn ecommerce_orders(
        &self,
        id: impl AsRef<str>,
        shop_id: impl AsRef<str>,
        query: Parameter,
    ) -> Response {
        let url: String = format!(
            "{}{}/{}/reports/ecommerce/orders",
            self.mailerlite.base_url,
            END_POINT,
            id.as_ref()
        );
        let query: Vec<(&str, &str)> = query
            .data
            .into_iter()
            .filter(|(key, _)| *key != "shop_id")
            .collect();

        Response::new(
            self.mailerlite
                .client
                .request
                .get(url)
                .query(&[("shop_id", shop_id.as_ref())])
                .query(&query)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }
}
