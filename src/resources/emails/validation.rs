//! Local path and extension-key checks for editor requests.
use crate::error::Error;
use serde_json::{Map, Value};

/// The API decodes an escaped "/" in the path, so URL encoding alone does not stop "123%2Fhtml".
pub(crate) fn path_id(name: &'static str, value: &str) -> Result<(), Error> {
    if value.is_empty() {
        return Err(Error::Config(format!("{name} must not be empty")));
    }
    if !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(Error::Config(format!(
            "{name} must contain only ASCII digits"
        )));
    }

    Ok(())
}

/// The SDK adds the etag itself.
pub(crate) fn no_etag_key(extra: &Map<String, Value>) -> Result<(), Error> {
    if extra.contains_key("etag") {
        return Err(Error::Config(
            "settings must not contain an etag key".into(),
        ));
    }

    Ok(())
}

/// A key that is also a typed field at the same level would go into the body two times.
pub(crate) fn no_field_keys(
    extra: &Map<String, Value>,
    fields: &[&str],
    path: &str,
) -> Result<(), Error> {
    match extra.keys().find(|key| fields.contains(&key.as_str())) {
        Some(key) => Err(Error::Config(format!(
            "duplicate settings key: {}",
            settings_path(path, key)
        ))),
        None => Ok(()),
    }
}

pub(crate) fn settings_path(path: &str, key: &str) -> String {
    if path.is_empty() {
        key.to_string()
    } else {
        format!("{path}.{key}")
    }
}

/// An empty string stays, and the SDK sends it as `key=`.
pub(crate) fn query_pairs<'a>(
    pairs: impl IntoIterator<Item = (&'a str, Option<String>)>,
) -> Vec<(&'a str, String)> {
    pairs
        .into_iter()
        .filter_map(|(key, value)| value.map(|value| (key, value)))
        .collect()
}
