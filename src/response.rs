use reqwest::{Response as reqwestResponse, StatusCode};
use serde_json::{json, Value};

#[derive(Debug)]
pub struct Response {
    pub status_code: StatusCode,
    pub content: Value,
}

impl Response {
    pub async fn new(response: reqwestResponse) -> Self {
        let status_code: StatusCode = response.status().clone();
        let content: Value = match status_code {
            StatusCode::NO_CONTENT => json!({
                "message": "No content",
            }),
            _ => response
                .json::<Value>()
                .await
                .expect("Failed to parse response body"),
        };

        Self {
            status_code,
            content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Client;
    use mockito::{Mock, Server, ServerGuard};

    #[tokio::test]
    async fn response_new() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("GET", "/hello")
            .with_status(200)
            .with_header("Accept", "application/json")
            .with_header("Content-Type", "application/json")
            .with_header("Authorization", "Bearer api_key")
            .with_body(r#"{"message": "Hello world"}"#)
            .create();

        let response: Response = Response::new(
            Client::new("api_key".to_string())
                .request
                .get(format!("{}/hello", server.url()))
                .send()
                .await
                .expect("Failed to send request"),
        )
        .await;

        mock.assert();
        assert_eq!(response.status_code, 200);
        assert_eq!(response.content, json!({"message": "Hello world"}));
    }
}
