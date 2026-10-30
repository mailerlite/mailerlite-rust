use crate::MailerLite;
use mockito::{Matcher, Mock, ServerGuard};

pub(crate) fn mailerlite(server: &ServerGuard) -> MailerLite {
    MailerLite::new("api_key".to_string()).with_base_url(server.url())
}

pub(crate) async fn mock_without_body(
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
