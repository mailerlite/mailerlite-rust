//! Shared wire checks for typed editor documents.
use super::content::BlockContent;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

/// Keep the nested field path when a tagged document is decoded from buffered JSON.
pub(super) fn decode<T: serde::de::DeserializeOwned>(value: Value) -> Result<T, String> {
    decode_at(value, "")
}

pub(super) fn decode_at<T: serde::de::DeserializeOwned>(
    value: Value,
    prefix: &str,
) -> Result<T, String> {
    serde_path_to_error::deserialize(value).map_err(|error| {
        let path = error.path().to_string();
        let location = match (prefix.is_empty(), path.as_str()) {
            (_, "" | ".") => prefix.to_owned(),
            (true, _) => path,
            (false, _) => format!("{prefix}.{path}"),
        };
        if location.is_empty() {
            error.inner().to_string()
        } else {
            format!("{location}: {}", error.inner())
        }
    })
}

pub(super) fn null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Option::<T>::deserialize(deserializer).map(Option::unwrap_or_default)
}

// Envelope used only when callers explicitly decode an optional typed helper.
#[derive(Deserialize)]
pub(super) struct BlockEnvelope {
    #[serde(default)]
    pub(super) id: Option<String>,
    #[serde(rename = "type")]
    kind: String,
    #[serde(default = "empty_object")]
    editor_content: Value,
}

impl BlockEnvelope {
    pub(super) fn decode_content(&mut self) -> Result<BlockContent, String> {
        BlockContent::from_document(&self.kind, std::mem::take(&mut self.editor_content)).map_err(
            |error| match &self.id {
                Some(id) => format!("block {id:?} ({:?}): {error}", self.kind),
                None => format!("block type {:?}: {error}", self.kind),
            },
        )
    }
}

pub(super) fn empty_object() -> Value {
    Value::Object(Map::new())
}

pub(super) fn serialize_extensions<S: Serializer>(
    extra: &Map<String, Value>,
    reserved: &[&str],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(key) = extra.keys().find(|key| reserved.contains(&key.as_str())) {
        return Err(serde::ser::Error::custom(format!(
            "duplicate editor field: {key}"
        )));
    }
    extra.serialize(serializer)
}

pub(super) fn tagged_object<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<(String, Map<String, Value>), D::Error> {
    let mut object = Map::<String, Value>::deserialize(deserializer)?;
    let Some(Value::String(kind)) = object.remove("type") else {
        return Err(serde::de::Error::custom("editor type must be a string"));
    };
    Ok((kind, object))
}

pub(super) fn object(value: Value) -> Result<Map<String, Value>, serde_json::Error> {
    match value {
        Value::Object(fields) => Ok(fields),
        Value::Array(items) if items.is_empty() => Ok(Map::new()),
        _ => Err(serde::de::Error::custom(
            "editor content must be an object or an empty array",
        )),
    }
}

#[derive(Serialize)]
pub(super) struct Tagged<'a, T> {
    #[serde(rename = "type")]
    pub(super) kind: &'a str,
    #[serde(flatten)]
    pub(super) fields: &'a T,
}

#[derive(Serialize)]
pub(super) struct TaggedBlock<'a, T> {
    #[serde(rename = "type")]
    pub(super) kind: &'a str,
    pub(super) editor_content: &'a T,
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert synthetic decoding diagnostics"
)]
mod tests {
    use super::decode_at;
    use serde::Deserialize;

    #[test]
    fn document_root_errors_use_a_separator_instead_of_a_fake_field_path() {
        #[derive(Debug, Deserialize)]
        struct Required {
            #[serde(rename = "required_field")]
            _field: String,
        }
        let detail = decode_at::<Required>(serde_json::json!({}), "editor_content").unwrap_err();
        assert!(detail.starts_with("editor_content: "), "{detail}");
    }
}
