use crate::{
    automation::{send, url},
    data::Data,
    parameter::Parameter,
    response::Response,
    MailerLite,
};

#[derive(Debug)]
pub struct AutomationTrigger {
    mailerlite: MailerLite,
}

impl AutomationTrigger {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, automation_id: impl AsRef<str>) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/triggers", automation_id.as_ref()),
        );

        send(self.mailerlite.client.request.get(url)).await
    }

    pub async fn find(
        &self,
        automation_id: impl AsRef<str>,
        trigger_id: impl AsRef<str>,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!(
                "{}/triggers/{}",
                automation_id.as_ref(),
                trigger_id.as_ref()
            ),
        );

        send(self.mailerlite.client.request.get(url)).await
    }

    pub async fn create(&self, automation_id: impl AsRef<str>, form: Data) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/triggers", automation_id.as_ref()),
        );

        send(self.mailerlite.client.request.post(url).form(&form.data)).await
    }

    pub async fn update(
        &self,
        automation_id: impl AsRef<str>,
        trigger_id: impl AsRef<str>,
        form: Data,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!(
                "{}/triggers/{}",
                automation_id.as_ref(),
                trigger_id.as_ref()
            ),
        );

        send(self.mailerlite.client.request.put(url).form(&form.data)).await
    }

    pub async fn delete(
        &self,
        automation_id: impl AsRef<str>,
        trigger_id: impl AsRef<str>,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!(
                "{}/triggers/{}",
                automation_id.as_ref(),
                trigger_id.as_ref()
            ),
        );

        send(self.mailerlite.client.request.delete(url)).await
    }

    pub async fn run_counts(&self, automation_id: impl AsRef<str>) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/triggers/run-counts", automation_id.as_ref()),
        );

        send(self.mailerlite.client.request.get(url)).await
    }

    pub async fn run_count(
        &self,
        automation_id: impl AsRef<str>,
        trigger_id: impl AsRef<str>,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!(
                "{}/triggers/run-counts/{}",
                automation_id.as_ref(),
                trigger_id.as_ref()
            ),
        );

        send(self.mailerlite.client.request.get(url)).await
    }

    pub async fn subscribers(
        &self,
        automation_id: impl AsRef<str>,
        trigger_id: impl AsRef<str>,
        query: Parameter,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!(
                "{}/activity/triggers/{}/subscribers",
                automation_id.as_ref(),
                trigger_id.as_ref()
            ),
        );

        send(self.mailerlite.client.request.get(url).query(&query.data)).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::Data,
        parameter::Parameter,
        response::Response,
        test_support::{mailerlite, mock_without_body},
    };
    use mockito::{Matcher, Mock, Server, ServerGuard};

    #[tokio::test]
    async fn get() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock =
            mock_without_body(&mut server, "GET", "/automations/1/triggers", 200).await;

        let response: Response = mailerlite(&server).automation_trigger().get("1").await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn find() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock =
            mock_without_body(&mut server, "GET", "/automations/1/triggers/2", 200).await;

        let response: Response = mailerlite(&server)
            .automation_trigger()
            .find("1", "2")
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn create() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("POST", "/automations/1/triggers")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("type".into(), "subscriber_joins_group".into()),
                Matcher::UrlEncoded("data[group_ids][0]".into(), "3".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation_trigger()
            .create(
                "1",
                Data::new()
                    .add("type", "subscriber_joins_group")
                    .add("data[group_ids][0]", "3"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn update() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("PUT", "/automations/1/triggers/2")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::UrlEncoded("data[group_ids][0]".into(), "3".into()))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation_trigger()
            .update("1", "2", Data::new().add("data[group_ids][0]", "3"))
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn delete() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock =
            mock_without_body(&mut server, "DELETE", "/automations/1/triggers/2", 204).await;

        let response: Response = mailerlite(&server)
            .automation_trigger()
            .delete("1", "2")
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 204);
    }

    #[tokio::test]
    async fn run_counts() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = mock_without_body(
            &mut server,
            "GET",
            "/automations/1/triggers/run-counts",
            200,
        )
        .await;

        let response: Response = mailerlite(&server)
            .automation_trigger()
            .run_counts("1")
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn run_count() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = mock_without_body(
            &mut server,
            "GET",
            "/automations/1/triggers/run-counts/2",
            200,
        )
        .await;

        let response: Response = mailerlite(&server)
            .automation_trigger()
            .run_count("1", "2")
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn subscribers() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("GET", "/automations/1/activity/triggers/2/subscribers")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("status".into(), "completed".into()),
                Matcher::UrlEncoded("limit".into(), "10".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation_trigger()
            .subscribers(
                "1",
                "2",
                Parameter::new()
                    .add("status", "completed")
                    .add("limit", "10"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }
}
