use crate::{parameter::Parameter, response::Response, MailerLite, BASE_PATH};

const END_POINT: &str = "automations";

#[derive(Debug)]
pub struct Automation {
    mailerlite: MailerLite,
}

impl Automation {
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

    pub async fn subscribers_activity(&self, id: String, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/activity", BASE_PATH, END_POINT, id);

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

    pub async fn ecommerce_stats(&self, id: String, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/monthly-totals/ecommerce", BASE_PATH, END_POINT, id);

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

    pub async fn email_ecommerce_stats(&self, id: String, email_id: String, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/emails/{}/ecommerce-overview-stats", BASE_PATH, END_POINT, id, email_id);

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

    pub async fn ecommerce_orders(&self, id: String, query: Parameter) -> Response {
        let url: String = format!("{}{}/{}/order-activity", BASE_PATH, END_POINT, id);

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
}
