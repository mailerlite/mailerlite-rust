//! Lazy cursor and numbered pagination with bounded continuation state.
use super::{CursorPage, NumberedPage};
use crate::{Error, MailerLite, Result, http::OperationPolicy};
use futures_core::Stream;
use futures_util::stream::try_unfold;
use reqwest::Method;
use serde::de::DeserializeOwned;

pub(crate) fn validated_stream<'a, T: 'a>(
    validation: crate::Result<()>,
    stream: impl futures_core::Stream<Item = crate::Result<T>> + 'a,
) -> impl futures_core::Stream<Item = crate::Result<T>> + 'a {
    use futures_util::StreamExt;
    let error = validation.err();
    let count = if error.is_none() { usize::MAX } else { 0 };
    futures_util::stream::iter(error.into_iter().map(Err)).chain(stream.take(count))
}

// Brent's cycle detector retains a single checkpoint, rather than every cursor.
// It catches arbitrary finite cycles while keeping stream memory bounded.
#[derive(Default)]
struct CursorCycle {
    checkpoint: Option<String>,
    power: usize,
    distance: usize,
}

impl CursorCycle {
    fn observe(&mut self, cursor: &str) -> Result<()> {
        if self.checkpoint.as_deref() == Some(cursor) {
            return Err(Error::Pagination(
                "server repeated a cursor continuation".into(),
            ));
        }
        if self.checkpoint.is_none() || self.distance == self.power {
            self.checkpoint = Some(cursor.to_owned());
            self.power = self.power.saturating_mul(2).max(1);
            self.distance = 0;
        }
        self.distance = self.distance.saturating_add(1);
        Ok(())
    }
}

struct State<'a, T> {
    client: &'a MailerLite,
    path: Vec<String>,
    query: Vec<(String, String)>,
    items: std::vec::IntoIter<T>,
    finished: bool,
    cycle: CursorCycle,
}

fn set_parameter(query: &mut Vec<(String, String)>, name: &str, value: String) {
    query.retain(|(key, _)| key != name);
    query.push((name.to_owned(), value));
}

pub(crate) fn cursor_stream<'a, T: DeserializeOwned + 'a>(
    client: &'a MailerLite,
    path: Vec<String>,
    query: Vec<(String, String)>,
) -> impl Stream<Item = Result<T>> + 'a {
    let state = State {
        client,
        path,
        query,
        items: Vec::new().into_iter(),
        finished: false,
        cycle: CursorCycle::default(),
    };
    try_unfold(state, |mut state| async move {
        loop {
            if let Some(item) = state.items.next() {
                return Ok(Some((item, state)));
            }
            if state.finished {
                return Ok(None);
            }
            if let Some((_, cursor)) = state.query.iter().find(|(key, _)| key == "cursor") {
                state.cycle.observe(cursor)?;
            }
            let path: Vec<&str> = state.path.iter().map(String::as_str).collect();
            let response = state
                .client
                .request::<CursorPage<T>>(
                    Method::GET,
                    &path,
                    &state.query,
                    None,
                    OperationPolicy::READ,
                )
                .await?;
            let page = response.data;
            match page.meta.next_cursor {
                Some(next) => set_parameter(&mut state.query, "cursor", next),
                None => state.finished = true,
            }
            state.items = page.data.into_iter();
        }
    })
}

pub(crate) fn numbered_stream<'a, T: DeserializeOwned + 'a>(
    client: &'a MailerLite,
    path: Vec<String>,
    query: Vec<(String, String)>,
) -> impl Stream<Item = Result<T>> + 'a {
    numbered_stream_with_policy(client, path, query, OperationPolicy::READ)
}

pub(crate) fn numbered_stream_with_policy<'a, T: DeserializeOwned + 'a>(
    client: &'a MailerLite,
    path: Vec<String>,
    query: Vec<(String, String)>,
    policy: OperationPolicy,
) -> impl Stream<Item = Result<T>> + 'a {
    let state = State {
        client,
        path,
        query,
        items: Vec::new().into_iter(),
        finished: false,
        cycle: CursorCycle::default(),
    };
    try_unfold(state, move |mut state| {
        let policy = policy.clone();
        async move {
            loop {
                if let Some(item) = state.items.next() {
                    return Ok(Some((item, state)));
                }
                if state.finished {
                    return Ok(None);
                }
                let requested = state
                    .query
                    .iter()
                    .find(|(key, _)| key == "page")
                    .map(|(_, value)| value.parse::<u64>())
                    .transpose()
                    .map_err(|_| Error::Pagination("invalid requested page number".into()))?
                    .unwrap_or(1);
                if requested == 0 {
                    return Err(Error::Pagination("page numbers start at one".into()));
                }
                let path: Vec<&str> = state.path.iter().map(String::as_str).collect();
                let response = state
                    .client
                    .request::<NumberedPage<T>>(
                        Method::GET,
                        &path,
                        &state.query,
                        None,
                        policy.clone(),
                    )
                    .await?;
                let page = response.data;
                if page
                    .meta
                    .current_page
                    .is_some_and(|current| current != requested)
                {
                    return Err(Error::Pagination(
                        "server returned an unexpected page number".into(),
                    ));
                }
                match page.next_page() {
                    Some(next) => set_parameter(&mut state.query, "page", next.to_string()),
                    None => state.finished = true,
                }
                state.items = page.data.into_iter();
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_cycles_are_detected_with_constant_storage() {
        for sequence in [
            vec!["a", "a"],
            vec!["a", "b", "c", "b", "c", "b", "c", "b", "c"],
            vec!["a", "b", "a", "b", "a", "b"],
        ] {
            let mut detector = CursorCycle::default();
            assert!(
                sequence
                    .into_iter()
                    .any(|cursor| detector.observe(cursor).is_err())
            );
        }
    }

    #[test]
    fn pagination_parameters_preserve_filters_and_remove_duplicates() {
        let mut query = vec![
            ("filter[status]".into(), "active".into()),
            ("cursor".into(), "old".into()),
            ("cursor".into(), "duplicate".into()),
            ("limit".into(), "100".into()),
        ];
        set_parameter(&mut query, "cursor", "new".into());
        assert_eq!(
            query,
            vec![
                ("filter[status]".into(), "active".into()),
                ("limit".into(), "100".into()),
                ("cursor".into(), "new".into())
            ]
        );
    }
}
