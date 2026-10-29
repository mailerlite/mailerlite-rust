use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client as reqwestClient,
};

#[derive(Debug, Clone)]
pub struct Client {
    pub request: reqwestClient,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Sensitive values print as "Sensitive" in Debug output, so the key can't leak
        // through `{:?}` on Client or MailerLite.
        let mut authorization: HeaderValue = HeaderValue::from_str(&format!("Bearer {api_key}"))
            .expect("API key must contain only visible ASCII characters");
        authorization.set_sensitive(true);
        headers.insert(AUTHORIZATION, authorization);

        Self {
            request: reqwestClient::builder()
                .default_headers(headers)
                .build()
                .expect("Failed to build client"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Mock, Server, ServerGuard};

    #[tokio::test]
    async fn client_new() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("GET", "/hello")
            .with_status(200)
            .match_header("Accept", "application/json")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer api_key")
            .with_body(r#"{"message": "Hello world"}"#)
            .create();

        let client: Client = Client::new("api_key".to_string());

        assert_eq!(
            client
                .request
                .get(format!("{}/hello", server.url()))
                .send()
                .await
                .expect("Failed to send request")
                .status(),
            200,
        );
        mock.assert();
        assert!(mock.matched());
    }
}
