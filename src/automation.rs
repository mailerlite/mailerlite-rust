use crate::{data::Data, parameter::Parameter, response::Response, MailerLite};

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

    pub async fn subscribers_activity(&self, id: impl AsRef<str>, query: Parameter) -> Response {
        let url: String = format!(
            "{}{}/{}/activity",
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

    pub async fn ecommerce_stats(&self, id: impl AsRef<str>, query: Parameter) -> Response {
        let url: String = format!(
            "{}{}/{}/monthly-totals/ecommerce",
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

    pub async fn email_ecommerce_stats(
        &self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
        query: Parameter,
    ) -> Response {
        let url: String = format!(
            "{}{}/{}/emails/{}/ecommerce-overview-stats",
            self.mailerlite.base_url,
            END_POINT,
            id.as_ref(),
            email_id.as_ref()
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

    pub async fn ecommerce_orders(&self, id: impl AsRef<str>, query: Parameter) -> Response {
        let url: String = format!(
            "{}{}/{}/order-activity",
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

    pub async fn update(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = url(&self.mailerlite, id.as_ref());

        send(self.mailerlite.client.request.put(url).form(&form.data)).await
    }

    pub async fn copy(&self, id: impl AsRef<str>) -> Response {
        let url: String = url(&self.mailerlite, &format!("{}/copy", id.as_ref()));

        send(self.mailerlite.client.request.post(url)).await
    }

    pub async fn enable(&self, id: impl AsRef<str>) -> Response {
        let url: String = url(&self.mailerlite, &format!("{}/enable", id.as_ref()));

        send(self.mailerlite.client.request.post(url)).await
    }

    pub async fn disable(&self, id: impl AsRef<str>) -> Response {
        let url: String = url(&self.mailerlite, &format!("{}/disable", id.as_ref()));

        send(self.mailerlite.client.request.post(url)).await
    }

    pub async fn test(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = url(&self.mailerlite, &format!("{}/test", id.as_ref()));

        send(self.mailerlite.client.request.post(url).form(&form.data)).await
    }

    pub async fn add_subscribers(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/add-subscribers", id.as_ref()),
        );

        send(self.mailerlite.client.request.post(url).form(&form.data)).await
    }

    pub async fn readd_subscribers(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = url(&self.mailerlite, &format!("{}/readd", id.as_ref()));

        send(self.mailerlite.client.request.post(url).form(&form.data)).await
    }

    pub async fn add_subscribers_to_group(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/activity/add-to-group", id.as_ref()),
        );

        send(self.mailerlite.client.request.post(url).form(&form.data)).await
    }

    pub async fn cancel_subscribers(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/activity/cancel-automation", id.as_ref()),
        );

        send(self.mailerlite.client.request.post(url).form(&form.data)).await
    }

    pub async fn skip_delay(&self, id: impl AsRef<str>, form: Data) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/activity/skip-delay", id.as_ref()),
        );

        send(self.mailerlite.client.request.post(url).form(&form.data)).await
    }

    pub async fn email_content(&self, id: impl AsRef<str>, email_id: impl AsRef<str>) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/emails/{}/content", id.as_ref(), email_id.as_ref()),
        );

        send(self.mailerlite.client.request.get(url)).await
    }

    pub async fn update_email_content(
        &self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
        form: Data,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/emails/{}/content", id.as_ref(), email_id.as_ref()),
        );

        send(self.mailerlite.client.request.put(url).form(&form.data)).await
    }

    pub async fn email_report_stats(
        &self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
        query: Parameter,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/emails/{}/report-stats", id.as_ref(), email_id.as_ref()),
        );

        send(self.mailerlite.client.request.get(url).query(&query.data)).await
    }

    pub async fn email_unsubscribe_reasons(
        &self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!(
                "{}/emails/{}/unsubscribe-reasons",
                id.as_ref(),
                email_id.as_ref()
            ),
        );

        send(self.mailerlite.client.request.get(url)).await
    }

    pub async fn email_stats(&self, id: impl AsRef<str>, query: Parameter) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/monthly-totals/email", id.as_ref()),
        );

        send(self.mailerlite.client.request.get(url).query(&query.data)).await
    }
}

pub(crate) fn url(mailerlite: &MailerLite, path: &str) -> String {
    format!("{}{}/{}", mailerlite.base_url, END_POINT, path)
}

pub(crate) async fn send(request: reqwest::RequestBuilder) -> Response {
    Response::new(request.send().await.expect("Failed to send request")).await
}

#[cfg(test)]
mod tests {
    use crate::{data::Data, parameter::Parameter, response::Response, MailerLite};
    use mockito::{Matcher, Mock, Server, ServerGuard};

    fn mailerlite(server: &ServerGuard) -> MailerLite {
        MailerLite::new("api_key".to_string()).with_base_url(server.url())
    }

    async fn mock_without_body(
        server: &mut ServerGuard,
        method: &str,
        path: &str,
        status: usize,
    ) -> Mock {
        server
            .mock(method, path)
            .match_query(Matcher::Missing)
            .with_status(status)
            .with_body(if status == 204 { "" } else { "{}" })
            .create_async()
            .await
    }

    #[tokio::test]
    async fn get() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("GET", "/automations")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("filter[enabled]".into(), "1".into()),
                Matcher::UrlEncoded("page".into(), "2".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .get(
                Parameter::new()
                    .add("filter[enabled]", "1")
                    .add("page", "2"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn find() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = mock_without_body(&mut server, "GET", "/automations/1", 200).await;

        let response: Response = mailerlite(&server).automation().find("1").await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn create() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("POST", "/automations")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::UrlEncoded(
                "name".into(),
                "Dummy Automation".into(),
            ))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .create(Data::new().add("name", "Dummy Automation"))
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn update() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("PUT", "/automations/1")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::UrlEncoded(
                "name".into(),
                "Dummy Automation".into(),
            ))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .update("1", Data::new().add("name", "Dummy Automation"))
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn delete() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = mock_without_body(&mut server, "DELETE", "/automations/1", 204).await;

        let response: Response = mailerlite(&server).automation().delete("1").await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 204);
    }

    #[tokio::test]
    async fn copy() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = mock_without_body(&mut server, "POST", "/automations/1/copy", 200).await;

        let response: Response = mailerlite(&server).automation().copy("1").await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn enable() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = mock_without_body(&mut server, "POST", "/automations/1/enable", 200).await;

        let response: Response = mailerlite(&server).automation().enable("1").await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn disable() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock =
            mock_without_body(&mut server, "POST", "/automations/1/disable", 200).await;

        let response: Response = mailerlite(&server).automation().disable("1").await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn test() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("POST", "/automations/1/test")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::UrlEncoded(
                "email".into(),
                "dummy@example.com".into(),
            ))
            .with_status(204)
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .test("1", Data::new().add("email", "dummy@example.com"))
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 204);
    }

    #[tokio::test]
    async fn subscribers_activity() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("GET", "/automations/1/activity")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("filter[status]".into(), "completed".into()),
                Matcher::UrlEncoded("filter[search]".into(), "dummy@example.com".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .subscribers_activity(
                "1",
                Parameter::new()
                    .add("filter[status]", "completed")
                    .add("filter[search]", "dummy@example.com"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn add_subscribers() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("POST", "/automations/1/add-subscribers")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("emails[0]".into(), "dummy@example.com".into()),
                Matcher::UrlEncoded("groups[0]".into(), "3".into()),
                Matcher::UrlEncoded("step_id".into(), "3".into()),
            ]))
            .with_status(204)
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .add_subscribers(
                "1",
                Data::new()
                    .add("emails[0]", "dummy@example.com")
                    .add("groups[0]", "3")
                    .add("step_id", "3"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 204);
    }

    #[tokio::test]
    async fn readd_subscribers() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("POST", "/automations/1/readd")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("readd_subscribers_to".into(), "last_completed_step".into()),
                Matcher::UrlEncoded("filter[status]".into(), "completed".into()),
            ]))
            .with_status(204)
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .readd_subscribers(
                "1",
                Data::new()
                    .add("readd_subscribers_to", "last_completed_step")
                    .add("filter[status]", "completed"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 204);
    }

    #[tokio::test]
    async fn add_subscribers_to_group() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("POST", "/automations/1/activity/add-to-group")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("destination_groups[0]".into(), "3".into()),
                Matcher::UrlEncoded("filter[status]".into(), "active".into()),
            ]))
            .with_status(204)
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .add_subscribers_to_group(
                "1",
                Data::new()
                    .add("destination_groups[0]", "3")
                    .add("filter[status]", "active"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 204);
    }

    #[tokio::test]
    async fn cancel_subscribers() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("POST", "/automations/1/activity/cancel-automation")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("filter[status]".into(), "active".into()),
                Matcher::UrlEncoded("filter[id][0]".into(), "3".into()),
            ]))
            .with_status(204)
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .cancel_subscribers(
                "1",
                Data::new()
                    .add("filter[status]", "active")
                    .add("filter[id][0]", "3"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 204);
    }

    #[tokio::test]
    async fn skip_delay() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("POST", "/automations/1/activity/skip-delay")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("filter[status]".into(), "active".into()),
                Matcher::UrlEncoded("filter[next_step]".into(), "3".into()),
            ]))
            .with_status(204)
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .skip_delay(
                "1",
                Data::new()
                    .add("filter[status]", "active")
                    .add("filter[next_step]", "3"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 204);
    }

    #[tokio::test]
    async fn email_content() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock =
            mock_without_body(&mut server, "GET", "/automations/1/emails/2/content", 200).await;

        let response: Response = mailerlite(&server)
            .automation()
            .email_content("1", "2")
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn update_email_content() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("PUT", "/automations/1/emails/2/content")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::UrlEncoded("html".into(), "<p>Hello</p>".into()))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .update_email_content("1", "2", Data::new().add("html", "<p>Hello</p>"))
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn email_report_stats() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("GET", "/automations/1/emails/2/report-stats")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("start_date".into(), "2026-01-01".into()),
                Matcher::UrlEncoded("end_date".into(), "2026-01-31".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .email_report_stats(
                "1",
                "2",
                Parameter::new()
                    .add("start_date", "2026-01-01")
                    .add("end_date", "2026-01-31"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn email_ecommerce_stats() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("GET", "/automations/1/emails/2/ecommerce-overview-stats")
            .match_query(Matcher::UrlEncoded("shop_id".into(), "5".into()))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .email_ecommerce_stats("1", "2", Parameter::new().add("shop_id", "5"))
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn email_unsubscribe_reasons() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = mock_without_body(
            &mut server,
            "GET",
            "/automations/1/emails/2/unsubscribe-reasons",
            200,
        )
        .await;

        let response: Response = mailerlite(&server)
            .automation()
            .email_unsubscribe_reasons("1", "2")
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn email_stats() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("GET", "/automations/1/monthly-totals/email")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("start_date".into(), "2026-01-01".into()),
                Matcher::UrlEncoded("end_date".into(), "2026-01-31".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .email_stats(
                "1",
                Parameter::new()
                    .add("start_date", "2026-01-01")
                    .add("end_date", "2026-01-31"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn ecommerce_stats() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("GET", "/automations/1/monthly-totals/ecommerce")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("start_date".into(), "2026-01-01".into()),
                Matcher::UrlEncoded("end_date".into(), "2026-01-31".into()),
                Matcher::UrlEncoded("shop_id".into(), "5".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .ecommerce_stats(
                "1",
                Parameter::new()
                    .add("start_date", "2026-01-01")
                    .add("end_date", "2026-01-31")
                    .add("shop_id", "5"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn ecommerce_orders() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("GET", "/automations/1/order-activity")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("time_frame".into(), "all_time".into()),
                Matcher::UrlEncoded("ecommerce_shop_id".into(), "5".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation()
            .ecommerce_orders(
                "1",
                Parameter::new()
                    .add("time_frame", "all_time")
                    .add("ecommerce_shop_id", "5"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }
}
