//! Optional query parameters shared by resource requests.

/// An empty string stays, and the SDK sends it as `key=`.
pub(super) fn query_pairs<'a>(
    pairs: impl IntoIterator<Item = (&'a str, Option<String>)>,
) -> Vec<(&'a str, String)> {
    pairs
        .into_iter()
        .filter_map(|(key, value)| value.map(|value| (key, value)))
        .collect()
}
