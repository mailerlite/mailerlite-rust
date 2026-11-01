//! Shared wire checks for typed editor documents.
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

pub(super) fn optional_value<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
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
    let kind = object
        .remove("type")
        .and_then(|value| value.as_str().map(str::to_owned))
        .ok_or_else(|| serde::de::Error::custom("editor type must be a string"))?;
    Ok((kind, object))
}

pub(super) fn object(value: Value) -> Result<Map<String, Value>, serde_json::Error> {
    match value {
        Value::Array(items) if items.is_empty() => Ok(Map::new()),
        value => serde_json::from_value(value),
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
