use crate::error::Error;
use serde_json::{Map, Value};

/// The API decodes an escaped "/" in the path, so URL encoding alone does not stop "123%2Fhtml".
pub(crate) fn path_id(name: &'static str, value: &str) -> Result<(), Error> {
    if value.is_empty() {
        return Err(Error::EmptyId { name });
    }
    if !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(Error::InvalidId {
            name,
            value: value.to_string(),
        });
    }

    Ok(())
}

/// The SDK adds the etag itself.
pub(crate) fn no_etag_key(extra: &Map<String, Value>) -> Result<(), Error> {
    if extra.contains_key("etag") {
        return Err(Error::EtagInSettings);
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
        Some(key) => Err(Error::DuplicateSettingsKey {
            key: settings_path(path, key),
        }),
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn path_id_accepts_only_ascii_digits() {
        assert!(path_id("email_id", "123").is_ok());

        assert!(matches!(
            path_id("email_id", ""),
            Err(Error::EmptyId { name: "email_id" })
        ));
        for value in ["123/html", "123%2Fhtml", "123\n", "²", " 123", "-1", "1.0"] {
            match path_id("block_id", value) {
                Err(Error::InvalidId { name, value: got }) => {
                    assert_eq!(name, "block_id");
                    assert_eq!(got, value);
                }
                other => panic!("expected InvalidId for {value:?}, got {other:?}"),
            }
        }
    }

    #[test]
    fn settings_extra_must_not_have_an_etag_key() {
        let mut extra: Map<String, Value> = Map::new();
        assert!(no_etag_key(&extra).is_ok());

        extra.insert("etag".to_string(), json!("42"));
        assert!(matches!(no_etag_key(&extra), Err(Error::EtagInSettings)));
    }

    #[test]
    fn no_field_keys_gives_the_path_of_the_duplicate_key() {
        let mut extra: Map<String, Value> = Map::new();
        extra.insert("new_key".to_string(), json!(1));
        assert!(no_field_keys(&extra, &["color", "size"], "typography.link").is_ok());

        extra.insert("color".to_string(), json!("#000000"));
        match no_field_keys(&extra, &["color", "size"], "typography.link") {
            Err(Error::DuplicateSettingsKey { key, .. }) => {
                assert_eq!(key, "typography.link.color")
            }
            other => panic!("expected DuplicateSettingsKey, got {other:?}"),
        }
        match no_field_keys(&extra, &["color"], "") {
            Err(Error::DuplicateSettingsKey { key, .. }) => assert_eq!(key, "color"),
            other => panic!("expected DuplicateSettingsKey, got {other:?}"),
        }
    }

    #[test]
    fn query_pairs_skip_none_and_keep_empty_strings() {
        let pairs: Vec<(&str, String)> = query_pairs([
            ("filter[name]", Some("summer sale".to_string())),
            ("filter[type]", None),
            ("sort", Some(String::new())),
        ]);

        assert_eq!(
            pairs,
            vec![
                ("filter[name]", "summer sale".to_string()),
                ("sort", String::new()),
            ]
        );
    }
}
