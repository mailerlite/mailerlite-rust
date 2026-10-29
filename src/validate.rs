//! Shared checks for the new methods. A check that fails stops the request before the SDK sends it.

use crate::error::Error;
use serde_json::{Map, Value};

/// Checks an email, block or version ID. The ID must contain only the ASCII digits 0-9.
///
/// The API decodes an escaped "/" in the path, so URL encoding alone does not stop "123%2Fhtml".
/// The check uses bytes, so it does not accept Unicode digits such as "²". The types of the methods
/// already stop a null or a boolean ID.
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

/// Checks the etag of a write.
pub(crate) fn etag(value: &str) -> Result<(), Error> {
    if value.is_empty() {
        return Err(Error::EmptyEtag);
    }

    Ok(())
}

/// Checks an idempotency key: 1 to 255 printable ASCII characters (0x20-0x7E), without a space at the start
/// or the end. HTTP removes such a space, so the API would get a different key.
pub(crate) fn idempotency_key(key: &str) -> Result<(), Error> {
    let valid: bool = (1..=255).contains(&key.len())
        && key.bytes().all(|byte| (0x20..=0x7e).contains(&byte))
        && !key.starts_with(' ')
        && !key.ends_with(' ');

    if !valid {
        return Err(Error::InvalidIdempotencyKey);
    }

    Ok(())
}

/// Checks that a free-form settings map has no `etag` key. The SDK adds the etag itself.
pub(crate) fn no_etag_key(extra: &Map<String, Value>) -> Result<(), Error> {
    if extra.contains_key("etag") {
        return Err(Error::EtagInSettings);
    }

    Ok(())
}

/// Checks that no key of a settings `extra` map is also the JSON name of a typed field at the same level.
/// Both would go into the body, and the API would get the key two times. `path` is the path of the map.
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

/// Joins a settings path and a key with ".".
pub(crate) fn settings_path(path: &str, key: &str) -> String {
    if path.is_empty() {
        key.to_string()
    } else {
        format!("{path}.{key}")
    }
}

/// Keeps the pairs that have a value. An empty string stays, and the SDK sends it as `key=`.
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
    fn etag_must_not_be_empty() {
        assert!(etag("42").is_ok());
        assert!(matches!(etag(""), Err(Error::EmptyEtag)));
    }

    #[test]
    fn idempotency_key_accepts_1_to_255_printable_ascii_characters() {
        assert!(idempotency_key("a").is_ok());
        assert!(idempotency_key("key with inner spaces").is_ok());
        assert!(idempotency_key(&"k".repeat(255)).is_ok());
    }

    #[test]
    fn idempotency_key_rejects_bad_keys() {
        let too_long: String = "k".repeat(256);
        for key in [
            "",
            too_long.as_str(),
            "key\n",
            "key\t",
            "clé",
            "key\x7f",
            " key",
            "key ",
        ] {
            assert!(
                matches!(idempotency_key(key), Err(Error::InvalidIdempotencyKey)),
                "expected InvalidIdempotencyKey for {key:?}"
            );
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
