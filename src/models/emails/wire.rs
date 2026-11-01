//! Shared wire checks for typed editor documents.
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

/// Keep the nested field path when a tagged document is decoded from buffered JSON.
pub(super) fn decode<T: serde::de::DeserializeOwned>(value: Value) -> Result<T, String> {
    serde_path_to_error::deserialize(value).map_err(|error| error.to_string())
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
