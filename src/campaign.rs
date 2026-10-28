use crate::{data::Data, parameter::Parameter, response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "campaigns";

#[derive(Debug)]
pub struct Campaign {
    mailerlite: MailerLite,
}

impl Campaign {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, query: Parameter) -> Response {
        let url: String = format!("{}{}", BASE_PATH, END_POINT);

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

    pub async fn find(&self, id: String) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, id);

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
        let url: String = format!("{}{}", BASE_PATH, END_POINT);

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

    pub async fn update(&self, id: String, form: Data) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, id);

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

    pub async fn delete(&self, id: String) -> Response {
        let url: String = format!("{}{}/{}", BASE_PATH, END_POINT, id);

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

    pub async fn schedule(&self, id: String, form: Data) -> Response {
        let url: String = format!("{}{}/{}/schedule", BASE_PATH, END_POINT, id);

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

    pub async fn cancel(&self, id: String) -> Response {
        let url: String = format!("{}{}/{}/cancel", BASE_PATH, END_POINT, id);

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

    pub async fn subscribers_activity(&self, id: String, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/reports/subscriber-activity", BASE_PATH, END_POINT, id);

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
        let url: String = format!("{}{}/languages", BASE_PATH, END_POINT);

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

    pub async fn ecommerce_stats(&self, id: String, shop_id: String) -> Response {
        let url: String = format!("{}{}/{}/reports/ecommerce", BASE_PATH, END_POINT, id);

        Response::new(
            self.mailerlite
                .client
                .request
                .get(url)
                .query(&[("shop_id", shop_id)])
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }

    pub async fn ecommerce_orders(&self, id: String, shop_id: String, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/reports/ecommerce/orders", BASE_PATH, END_POINT, id);
        let query: Vec<(&str, &str)> = query.data.into_iter().filter(|(key, _)| *key != "shop_id").collect();

        Response::new(
            self.mailerlite
                .client
                .request
                .get(url)
                .query(&[("shop_id", shop_id)])
                .query(&query)
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await
    }
}
