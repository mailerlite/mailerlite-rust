use crate::{
    automation::{send, url},
    data::Data,
    parameter::Parameter,
    response::Response,
    MailerLite,
};

#[derive(Debug)]
pub struct AutomationStep {
    mailerlite: MailerLite,
}

impl AutomationStep {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    pub async fn get(&self, automation_id: impl AsRef<str>) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/steps/index", automation_id.as_ref()),
        );

        send(self.mailerlite.client.request.get(url)).await
    }

    pub async fn find(&self, automation_id: impl AsRef<str>, step_id: impl AsRef<str>) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/steps/{}", automation_id.as_ref(), step_id.as_ref()),
        );

        send(self.mailerlite.client.request.get(url)).await
    }

    pub async fn create(&self, automation_id: impl AsRef<str>, form: Data) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/steps", automation_id.as_ref()),
        );

        send(self.mailerlite.client.request.post(url).form(&form.data)).await
    }

    pub async fn update(
        &self,
        automation_id: impl AsRef<str>,
        step_id: impl AsRef<str>,
        form: Data,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/steps/{}", automation_id.as_ref(), step_id.as_ref()),
        );

        send(self.mailerlite.client.request.put(url).form(&form.data)).await
    }

    pub async fn delete(
        &self,
        automation_id: impl AsRef<str>,
        step_id: impl AsRef<str>,
        query: Parameter,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/steps/{}", automation_id.as_ref(), step_id.as_ref()),
        );

        send(
            self.mailerlite
                .client
                .request
                .delete(url)
                .query(&query.data),
        )
        .await
    }

    pub async fn copy(
        &self,
        automation_id: impl AsRef<str>,
        step_id: impl AsRef<str>,
        form: Data,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/steps/{}/copy", automation_id.as_ref(), step_id.as_ref()),
        );

        send(self.mailerlite.client.request.post(url).form(&form.data)).await
    }

    pub async fn reorder(&self, automation_id: impl AsRef<str>, form: Data) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/steps/reorder", automation_id.as_ref()),
        );

        send(self.mailerlite.client.request.post(url).form(&form.data)).await
    }

    pub async fn select_ab_test_winner(
        &self,
        automation_id: impl AsRef<str>,
        step_id: impl AsRef<str>,
        form: Data,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!(
                "{}/steps/{}/select-ab-test-winner",
                automation_id.as_ref(),
                step_id.as_ref()
            ),
        );

        send(self.mailerlite.client.request.put(url).form(&form.data)).await
    }

    pub async fn run_counts(&self, automation_id: impl AsRef<str>) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/steps/run-counts", automation_id.as_ref()),
        );

        send(self.mailerlite.client.request.get(url)).await
    }

    pub async fn run_count(
        &self,
        automation_id: impl AsRef<str>,
        step_id: impl AsRef<str>,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!(
                "{}/steps/run-counts/{}",
                automation_id.as_ref(),
                step_id.as_ref()
            ),
        );

        send(self.mailerlite.client.request.get(url)).await
    }

    pub async fn ab_stats(&self, automation_id: impl AsRef<str>) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!("{}/ab-steps-stats", automation_id.as_ref()),
        );

        send(self.mailerlite.client.request.get(url)).await
    }

    pub async fn ab_stat(
        &self,
        automation_id: impl AsRef<str>,
        step_id: impl AsRef<str>,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!(
                "{}/ab-steps-stats/{}",
                automation_id.as_ref(),
                step_id.as_ref()
            ),
        );

        send(self.mailerlite.client.request.get(url)).await
    }

    pub async fn subscribers(
        &self,
        automation_id: impl AsRef<str>,
        step_id: impl AsRef<str>,
        query: Parameter,
    ) -> Response {
        let url: String = url(
            &self.mailerlite,
            &format!(
                "{}/activity/steps/{}/subscribers",
                automation_id.as_ref(),
                step_id.as_ref()
            ),
        );

        send(self.mailerlite.client.request.get(url).query(&query.data)).await
    }
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
        let mock: Mock =
            mock_without_body(&mut server, "GET", "/automations/1/steps/index", 200).await;

        let response: Response = mailerlite(&server).automation_step().get("1").await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn find() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = mock_without_body(&mut server, "GET", "/automations/1/steps/2", 200).await;

        let response: Response = mailerlite(&server).automation_step().find("1", "2").await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn create() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("POST", "/automations/1/steps")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("type".into(), "delay".into()),
                Matcher::UrlEncoded("parent_id".into(), "3".into()),
                Matcher::UrlEncoded("data[unit]".into(), "days".into()),
                Matcher::UrlEncoded("data[value]".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation_step()
            .create(
                "1",
                Data::new()
                    .add("type", "delay")
                    .add("parent_id", "3")
                    .add("data[unit]", "days")
                    .add("data[value]", "1"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn update() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("PUT", "/automations/1/steps/2")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("data[unit]".into(), "days".into()),
                Matcher::UrlEncoded("data[value]".into(), "2".into()),
                Matcher::UrlEncoded("reschedule_waiting_subscribers".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation_step()
            .update(
                "1",
                "2",
                Data::new()
                    .add("data[unit]", "days")
                    .add("data[value]", "2")
                    .add("reschedule_waiting_subscribers", "1"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn delete() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("DELETE", "/automations/1/steps/2")
            .match_query(Matcher::UrlEncoded("branch_to_delete".into(), "all".into()))
            .with_status(204)
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation_step()
            .delete("1", "2", Parameter::new().add("branch_to_delete", "all"))
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 204);
    }

    #[tokio::test]
    async fn copy() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("POST", "/automations/1/steps/2/copy")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::UrlEncoded("parent_id".into(), "3".into()))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation_step()
            .copy("1", "2", Data::new().add("parent_id", "3"))
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn reorder() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("POST", "/automations/1/steps/reorder")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("dragged_step_id".into(), "3".into()),
                Matcher::UrlEncoded("new_parent_step_id".into(), "4".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation_step()
            .reorder(
                "1",
                Data::new()
                    .add("dragged_step_id", "3")
                    .add("new_parent_step_id", "4"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn select_ab_test_winner() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("PUT", "/automations/1/steps/2/select-ab-test-winner")
            .match_query(Matcher::Missing)
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(Matcher::UrlEncoded(
                "ab_test_winner".into(),
                "a_path".into(),
            ))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation_step()
            .select_ab_test_winner("1", "2", Data::new().add("ab_test_winner", "a_path"))
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn run_counts() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock =
            mock_without_body(&mut server, "GET", "/automations/1/steps/run-counts", 200).await;

        let response: Response = mailerlite(&server).automation_step().run_counts("1").await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn run_count() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock =
            mock_without_body(&mut server, "GET", "/automations/1/steps/run-counts/2", 200).await;

        let response: Response = mailerlite(&server)
            .automation_step()
            .run_count("1", "2")
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn ab_stats() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock =
            mock_without_body(&mut server, "GET", "/automations/1/ab-steps-stats", 200).await;

        let response: Response = mailerlite(&server).automation_step().ab_stats("1").await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn ab_stat() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock =
            mock_without_body(&mut server, "GET", "/automations/1/ab-steps-stats/2", 200).await;

        let response: Response = mailerlite(&server)
            .automation_step()
            .ab_stat("1", "2")
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn subscribers() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("GET", "/automations/1/activity/steps/2/subscribers")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("status".into(), "queued".into()),
                Matcher::UrlEncoded("filter[search]".into(), "dummy@example.com".into()),
            ]))
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let response: Response = mailerlite(&server)
            .automation_step()
            .subscribers(
                "1",
                "2",
                Parameter::new()
                    .add("status", "queued")
                    .add("filter[search]", "dummy@example.com"),
            )
            .await;

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
    }
}
