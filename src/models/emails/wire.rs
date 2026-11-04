//! Defaults for email response fields.
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

pub(super) fn null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Option::<T>::deserialize(deserializer).map(Option::unwrap_or_default)
}

pub(super) fn empty_object() -> Value {
    Value::Object(Map::new())
}
