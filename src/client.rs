use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client as reqwestClient,
};

#[derive(Debug, Clone)]
pub struct Client {
    pub request: reqwestClient,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", api_key).parse().unwrap(),
        );

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
