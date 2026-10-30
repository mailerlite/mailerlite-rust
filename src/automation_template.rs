use crate::{
    automation::{send, url},
    parameter::Parameter,
    response::Response,
    MailerLite,
};

const END_POINT: &str = "automation-templates";

#[derive(Debug)]
pub struct AutomationTemplate {
    mailerlite: MailerLite,
}

impl AutomationTemplate {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, query: Parameter) -> Response {
        let url: String = format!("{}{}", self.mailerlite.base_url, END_POINT);

        send(self.mailerlite.client.request.get(url).query(&query.data)).await
    }

    pub async fn select(&self, template_id: impl AsRef<str>) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("select-template/{}", template_id.as_ref()),
        );

        send(self.mailerlite.client.request.post(url)).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{parameter::Parameter, response::Response, MailerLite};
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
            .mock("GET", "/automation-templates")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("filter[type]".into(), "welcome".into()),
                Matcher::UrlEncoded("limit".into(), "10".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation_template()
            .get(
                Parameter::new()
                    .add("filter[type]", "welcome")
                    .add("limit", "10"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn select() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock =
            mock_without_body(&mut server, "POST", "/automations/select-template/3", 200).await;

        let response: Response = mailerlite(&server).automation_template().select("3").await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }
}
