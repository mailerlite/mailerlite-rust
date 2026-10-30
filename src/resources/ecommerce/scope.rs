//! Internal paths and query state shared by shop-scoped resources.
use crate::{MailerLite, Response, Result, http::OperationPolicy, response::Count};
use reqwest::Method;

#[derive(Clone, Debug)]
pub(super) struct Scope<'a> {
    pub(super) client: &'a MailerLite,
    pub(super) path: Vec<String>,
    pub(super) query: Vec<(String, String)>,
}
impl<'a> Scope<'a> {
    pub(super) fn new(client: &'a MailerLite, path: Vec<String>) -> Self {
        Self {
            client,
            path,
            query: vec![],
        }
    }
    pub(super) fn child(&self, segment: &str) -> Self {
        let mut next = self.clone();
        next.path.push(segment.into());
        next
    }
    pub(super) async fn data<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        body: Option<serde_json::Value>,
    ) -> Result<Response<T>> {
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        let policy = if method == Method::GET {
            OperationPolicy::READ
        } else {
            OperationPolicy::WRITE
        };
        self.client
            .data(method, &path, &self.query, body, policy)
            .await
    }
    pub(super) async fn count(&self) -> Result<Response<Count>> {
        let mut query = self.query.clone();
        query.push(("limit".into(), "0".into()));
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        self.client
            .request(Method::GET, &path, &query, None, OperationPolicy::READ)
            .await
    }
}
