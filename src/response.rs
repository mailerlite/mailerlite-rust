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
