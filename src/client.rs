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
